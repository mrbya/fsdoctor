use std::{collections::HashMap, path::Path, sync::Arc};

use tokio::{
    sync::mpsc,
    task::{self, JoinHandle},
};

use crate::{
    db::scan::ScanCounters, hash_file, scan_tree, CancelToken, CheckResultKind, CheckResultRecord,
    Error, ExpectedManifestEntry, FsEntry, FsEntryKind, FsEntryStatus, HashOptions, HashOutcome,
    IntegrityCheckOptions, IntegrityCheckPhase, IntegrityCheckProgress, IntegrityCheckReport,
    IntegrityCheckSummary, ProjectDb, Result, ScanFlow, ScanId, ScanKind, ScanOptions, ScanStatus,
};

/// Number of check results buffered between producer and DB writer.
const CHECK_RESULT_CHANNEL_CAPACITY: usize = 1024;

/// Progress callback type used by the integrity-check engine.
type ProgressCallback = Arc<dyn Fn(IntegrityCheckProgress) + Send + Sync + 'static>;

/// Internal producer result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CheckProducerReport {
    /// Scan counters collected by the producer.
    counters: ScanCounters,

    /// Integrity-check result summary.
    summary: IntegrityCheckSummary,

    /// Stopped because of a cancellation request?
    cancelled: bool,
}

/// Runs an integrity check against the latest completed manifest.
///
/// # Errors
///
/// Returns an error if manifest loading, scanning, hashing, worker execution,
/// or database persistence fails.
pub async fn run_integrity_check(
    db: &ProjectDb,
    options: IntegrityCheckOptions,
    cancel_token: &CancelToken,
) -> Result<IntegrityCheckReport> {
    run_integrity_check_inner(db, options, cancel_token, None).await
}

/// Runs an integrity check against the latest completed manifest and reports progress.
///
/// # Errors
///
/// Returns an error if manifest loading, scanning, hashing, worker execution,
/// or database persistence fails.
pub async fn run_integrity_check_with_progress(
    db: &ProjectDb,
    options: IntegrityCheckOptions,
    cancel_token: &CancelToken,
    on_progress: impl Fn(IntegrityCheckProgress) + Send + Sync + 'static,
) -> Result<IntegrityCheckReport> {
    run_integrity_check_inner(db, options, cancel_token, Some(Arc::new(on_progress))).await
}

/// Internal integrity-check implementation.
async fn run_integrity_check_inner(
    db: &ProjectDb,
    options: IntegrityCheckOptions,
    cancel_token: &CancelToken,
    progress: Option<ProgressCallback>,
) -> Result<IntegrityCheckReport> {
    if options.db_batch_size == 0 {
        return Err(Error::InvalidIntegrityCheckBatchSize);
    }

    let project = db.project().await?;

    emit_progress(
        progress.as_ref(),
        IntegrityCheckPhase::LoadingManifest,
        None,
        IntegrityCheckSummary::default(),
        ScanCounters::default(),
        0,
    );

    let manifest_scan_id = db
        .latest_completed_manifest_scan(project.id)
        .await?
        .ok_or(Error::NoCompletedManifest)?;

    let expected_entries = db
        .load_manifest_for_check(project.id, manifest_scan_id)
        .await?;

    let check_scan_id = db.create_scan(project.id, ScanKind::IntegrityCheck).await?;

    let (sender, receiver) = mpsc::channel::<CheckResultRecord>(CHECK_RESULT_CHANNEL_CAPACITY);

    let root_path = project.root_path.clone();
    let producer_cancel_token = cancel_token.clone();
    let producer_progress = progress.clone();

    let producer = task::spawn_blocking(move || {
        produce_check_results(
            check_scan_id,
            &root_path,
            expected_entries,
            sender,
            &producer_cancel_token,
            producer_progress,
        )
    });

    let writer_result =
        consume_check_results(db, receiver, options.db_batch_size, progress.clone()).await;

    if writer_result.is_err() {
        cancel_token.cancel();
    }

    let producer_result = await_check_producer(producer).await;

    finish_integrity_check(
        db,
        check_scan_id,
        manifest_scan_id,
        producer_result,
        writer_result,
        progress.as_ref(),
    )
    .await
}

/// Awaits the blocking check producer task.
async fn await_check_producer(
    producer: JoinHandle<Result<CheckProducerReport>>,
) -> Result<CheckProducerReport> {
    producer
        .await
        .map_err(|source| Error::IntegrityCheckWorkerJoin { source })?
}

/// Finishes scan lifecycle and returns the public report.
async fn finish_integrity_check(
    db: &ProjectDb,
    check_scan_id: ScanId,
    manifest_scan_id: ScanId,
    producer_result: Result<CheckProducerReport>,
    writer_result: Result<()>,
    progress: Option<&ProgressCallback>,
) -> Result<IntegrityCheckReport> {
    match (producer_result, writer_result) {
        (Ok(producer_report), Ok(())) => {
            let status = if producer_report.cancelled {
                ScanStatus::Cancelled
            } else {
                ScanStatus::Completed
            };

            emit_progress(
                progress,
                IntegrityCheckPhase::Finishing,
                None,
                producer_report.summary,
                producer_report.counters,
                0,
            );

            db.finish_scan(check_scan_id, status, producer_report.counters, None)
                .await?;

            Ok(IntegrityCheckReport {
                scan_id: check_scan_id,
                manifest_scan_id,
                summary: producer_report.summary,
                cancelled: producer_report.cancelled,
            })
        }
        (Ok(producer_report), Err(error)) => {
            let message = error.to_string();

            db.finish_scan(
                check_scan_id,
                ScanStatus::Failed,
                producer_report.counters,
                Some(&message),
            )
            .await?;

            Err(error)
        }
        (Err(error), Ok(()) | Err(_)) => {
            let message = error.to_string();

            db.finish_scan(
                check_scan_id,
                ScanStatus::Failed,
                ScanCounters::default(),
                Some(&message),
            )
            .await?;

            Err(error)
        }
    }
}

/// Produces check results by walking, hashing, and comparing the filesystem tree.
///
/// This function is intended to run in a blocking worker thread.
fn produce_check_results(
    check_scan_id: ScanId,
    root_path: &Path,
    mut expected_entries: HashMap<String, ExpectedManifestEntry>,
    sender: mpsc::Sender<CheckResultRecord>,
    cancel_token: &CancelToken,
    progress: Option<ProgressCallback>,
) -> Result<CheckProducerReport> {
    let mut counters = ScanCounters::default();
    let mut summary = IntegrityCheckSummary::default();
    let mut cancelled = false;

    scan_tree(root_path, ScanOptions::default(), |entry| {
        if cancel_token.is_cancelled() {
            cancelled = true;
            return Ok(ScanFlow::Stop);
        }

        let current_path = entry.relative_path.as_str().to_owned();
        let expected = expected_entries.remove(&current_path);

        let result = classify_current_entry(
            check_scan_id,
            entry,
            expected,
            &mut counters,
            &mut summary,
            cancel_token,
        )?;

        emit_progress(
            progress.as_ref(),
            IntegrityCheckPhase::WalkingAndChecking,
            Some(current_path),
            summary,
            counters,
            0,
        );

        if sender.blocking_send(result).is_err() {
            cancelled = cancel_token.is_cancelled();
            return Ok(ScanFlow::Stop);
        }

        Ok(ScanFlow::Continue)
    })?;

    if !cancelled && !cancel_token.is_cancelled() {
        emit_progress(
            progress.as_ref(),
            IntegrityCheckPhase::RecordingMissingEntries,
            None,
            summary,
            counters,
            0,
        );

        for expected in expected_entries.into_values() {
            if cancel_token.is_cancelled() {
                cancelled = true;
                break;
            }

            let result = missing_result(check_scan_id, expected, &mut summary);

            if sender.blocking_send(result).is_err() {
                cancelled = cancel_token.is_cancelled();
                break;
            }
        }
    }

    Ok(CheckProducerReport {
        counters,
        summary,
        cancelled: cancelled || cancel_token.is_cancelled(),
    })
}

/// Consumes check results and writes them to `SQLite` in batches.
async fn consume_check_results(
    db: &ProjectDb,
    mut receiver: mpsc::Receiver<CheckResultRecord>,
    batch_size: usize,
    progress: Option<ProgressCallback>,
) -> Result<()> {
    let mut batch = Vec::with_capacity(batch_size);
    let mut results_written = 0_u64;

    while let Some(record) = receiver.recv().await {
        batch.push(record);

        if batch.len() >= batch_size {
            db.insert_check_result(&batch).await?;
            results_written = results_written.saturating_add(
                u64::try_from(batch.len()).map_err(|_error| Error::NumericOverflow)?,
            );
            batch.clear();

            emit_progress(
                progress.as_ref(),
                IntegrityCheckPhase::Writing,
                None,
                IntegrityCheckSummary::default(),
                ScanCounters::default(),
                results_written,
            );
        }
    }

    if !batch.is_empty() {
        db.insert_check_result(&batch).await?;
        results_written = results_written
            .saturating_add(u64::try_from(batch.len()).map_err(|_error| Error::NumericOverflow)?);
    }

    emit_progress(
        progress.as_ref(),
        IntegrityCheckPhase::Writing,
        None,
        IntegrityCheckSummary::default(),
        ScanCounters::default(),
        results_written,
    );

    Ok(())
}

/// Classifies one current filesystem entry against an optional expected entry.
fn classify_current_entry(
    check_scan_id: ScanId,
    entry: FsEntry,
    expected: Option<ExpectedManifestEntry>,
    counters: &mut ScanCounters,
    summary: &mut IntegrityCheckSummary,
    cancel_token: &CancelToken,
) -> Result<CheckResultRecord> {
    update_seen_counters(counters, &entry);

    let relative_path = entry.relative_path.as_str().to_owned();
    let actual_size = entry
        .metadata
        .as_ref()
        .and_then(|metadata| metadata.size_bytes);

    match &entry.status {
        FsEntryStatus::Unreadable { message } => {
            counters.unreadable_entries = counters.unreadable_entries.saturating_add(1);
            summary.unreadable = summary.unreadable.saturating_add(1);

            return Ok(CheckResultRecord {
                scan_id: check_scan_id,
                relative_path,
                result_kind: CheckResultKind::Unreadable,
                expected_entry_kind: expected.as_ref().map(|value| value.entry_kind),
                actual_entry_kind: Some(entry.kind),
                expected_size_bytes: expected.as_ref().and_then(|value| value.size_bytes),
                actual_size_bytes: actual_size,
                expected_hash: expected.as_ref().and_then(|value| value.hash),
                actual_hash: None,
                message: Some(message.clone()),
            });
        }
        FsEntryStatus::Skipped { reason } => {
            summary.skipped = summary.skipped.saturating_add(1);

            return Ok(CheckResultRecord {
                scan_id: check_scan_id,
                relative_path,
                result_kind: CheckResultKind::Skipped,
                expected_entry_kind: expected.as_ref().map(|value| value.entry_kind),
                actual_entry_kind: Some(entry.kind),
                expected_size_bytes: expected.as_ref().and_then(|value| value.size_bytes),
                actual_size_bytes: actual_size,
                expected_hash: expected.as_ref().and_then(|value| value.hash),
                actual_hash: None,
                message: Some(reason.to_str().to_owned()),
            });
        }
        FsEntryStatus::Accessible => {}
    }

    let Some(expected) = expected else {
        summary.new = summary.new.saturating_add(1);

        return Ok(CheckResultRecord {
            scan_id: check_scan_id,
            relative_path,
            result_kind: CheckResultKind::New,
            expected_entry_kind: None,
            actual_entry_kind: Some(entry.kind),
            expected_size_bytes: None,
            actual_size_bytes: actual_size,
            expected_hash: None,
            actual_hash: None,
            message: None,
        });
    };

    if expected.entry_kind != entry.kind {
        summary.type_changed = summary.type_changed.saturating_add(1);

        return Ok(CheckResultRecord {
            scan_id: check_scan_id,
            relative_path,
            result_kind: CheckResultKind::TypeChanged,
            expected_entry_kind: Some(expected.entry_kind),
            actual_entry_kind: Some(entry.kind),
            expected_size_bytes: expected.size_bytes,
            actual_size_bytes: actual_size,
            expected_hash: expected.hash,
            actual_hash: None,
            message: None,
        });
    }

    match entry.kind {
        FsEntryKind::File => classify_file_entry(
            check_scan_id,
            entry,
            expected,
            actual_size,
            counters,
            summary,
            cancel_token,
        ),
        FsEntryKind::Directory | FsEntryKind::Symlink | FsEntryKind::Other => {
            summary.ok = summary.ok.saturating_add(1);

            Ok(CheckResultRecord {
                scan_id: check_scan_id,
                relative_path,
                result_kind: CheckResultKind::Ok,
                expected_entry_kind: Some(expected.entry_kind),
                actual_entry_kind: Some(entry.kind),
                expected_size_bytes: expected.size_bytes,
                actual_size_bytes: actual_size,
                expected_hash: expected.hash,
                actual_hash: None,
                message: None,
            })
        }
    }
}

/// Classifies a regular file entry.
fn classify_file_entry(
    check_scan_id: ScanId,
    entry: FsEntry,
    expected: ExpectedManifestEntry,
    actual_size: Option<u64>,
    counters: &mut ScanCounters,
    summary: &mut IntegrityCheckSummary,
    cancel_token: &CancelToken,
) -> Result<CheckResultRecord> {
    if expected.size_bytes != actual_size {
        summary.size_mismatch = summary.size_mismatch.saturating_add(1);

        return Ok(CheckResultRecord {
            scan_id: check_scan_id,
            relative_path: expected.relative_path,
            result_kind: CheckResultKind::SizeMismatch,
            expected_entry_kind: Some(expected.entry_kind),
            actual_entry_kind: Some(entry.kind),
            expected_size_bytes: expected.size_bytes,
            actual_size_bytes: actual_size,
            expected_hash: expected.hash,
            actual_hash: None,
            message: None,
        });
    }

    match hash_file(&entry.absolute_path, HashOptions::default(), cancel_token) {
        Ok(HashOutcome::Hashed(hashed)) => {
            counters.hashed_files = counters.hashed_files.saturating_add(1);

            let actual_hash = hashed.digest.bytes;

            if expected.hash == Some(actual_hash) {
                summary.ok = summary.ok.saturating_add(1);

                Ok(CheckResultRecord {
                    scan_id: check_scan_id,
                    relative_path: expected.relative_path,
                    result_kind: CheckResultKind::Ok,
                    expected_entry_kind: Some(expected.entry_kind),
                    actual_entry_kind: Some(entry.kind),
                    expected_size_bytes: expected.size_bytes,
                    actual_size_bytes: Some(hashed.fingerprint.size_bytes),
                    expected_hash: expected.hash,
                    actual_hash: Some(actual_hash),
                    message: None,
                })
            } else {
                summary.hash_mismatch = summary.hash_mismatch.saturating_add(1);

                Ok(CheckResultRecord {
                    scan_id: check_scan_id,
                    relative_path: expected.relative_path,
                    result_kind: CheckResultKind::HashMismatch,
                    expected_entry_kind: Some(expected.entry_kind),
                    actual_entry_kind: Some(entry.kind),
                    expected_size_bytes: expected.size_bytes,
                    actual_size_bytes: Some(hashed.fingerprint.size_bytes),
                    expected_hash: expected.hash,
                    actual_hash: Some(actual_hash),
                    message: None,
                })
            }
        }
        Ok(HashOutcome::ChangedDuringHash { after, .. }) => {
            counters.changed_during_scan = counters.changed_during_scan.saturating_add(1);
            summary.changed_during_check = summary.changed_during_check.saturating_add(1);

            Ok(CheckResultRecord {
                scan_id: check_scan_id,
                relative_path: expected.relative_path,
                result_kind: CheckResultKind::ChangedDuringCheck,
                expected_entry_kind: Some(expected.entry_kind),
                actual_entry_kind: Some(entry.kind),
                expected_size_bytes: expected.size_bytes,
                actual_size_bytes: Some(after.size_bytes),
                expected_hash: expected.hash,
                actual_hash: None,
                message: Some("file changed while being checked".to_owned()),
            })
        }
        Err(Error::HashingCancelled) => Err(Error::HashingCancelled),
        Err(Error::FileSystem { source, .. }) => {
            counters.unreadable_entries = counters.unreadable_entries.saturating_add(1);
            summary.unreadable = summary.unreadable.saturating_add(1);

            Ok(CheckResultRecord {
                scan_id: check_scan_id,
                relative_path: expected.relative_path,
                result_kind: CheckResultKind::Unreadable,
                expected_entry_kind: Some(expected.entry_kind),
                actual_entry_kind: Some(entry.kind),
                expected_size_bytes: expected.size_bytes,
                actual_size_bytes: actual_size,
                expected_hash: expected.hash,
                actual_hash: None,
                message: Some(source.to_string()),
            })
        }
        Err(error) => Err(error),
    }
}

/// Creates a result for an expected entry missing from the current tree.
fn missing_result(
    check_scan_id: ScanId,
    expected: ExpectedManifestEntry,
    summary: &mut IntegrityCheckSummary,
) -> CheckResultRecord {
    summary.missing = summary.missing.saturating_add(1);

    CheckResultRecord {
        scan_id: check_scan_id,
        relative_path: expected.relative_path,
        result_kind: CheckResultKind::Missing,
        expected_entry_kind: Some(expected.entry_kind),
        actual_entry_kind: None,
        expected_size_bytes: expected.size_bytes,
        actual_size_bytes: None,
        expected_hash: expected.hash,
        actual_hash: None,
        message: None,
    }
}

/// Updates current-tree scan counters.
fn update_seen_counters(counters: &mut ScanCounters, entry: &FsEntry) {
    match entry.kind {
        FsEntryKind::File => {
            counters.total_files = counters.total_files.saturating_add(1);

            if let Some(size_bytes) = entry
                .metadata
                .as_ref()
                .and_then(|metadata| metadata.size_bytes)
            {
                counters.total_bytes = counters.total_bytes.saturating_add(size_bytes);
            }
        }
        FsEntryKind::Directory => {
            counters.total_dirs = counters.total_dirs.saturating_add(1);
        }
        FsEntryKind::Symlink => {
            counters.total_symlinks = counters.total_symlinks.saturating_add(1);
        }
        FsEntryKind::Other => {
            counters.total_other = counters.total_other.saturating_add(1);
        }
    }
}

/// Emits progress if a callback exists.
fn emit_progress(
    progress: Option<&ProgressCallback>,
    phase: IntegrityCheckPhase,
    current_path: Option<String>,
    summary: IntegrityCheckSummary,
    counters: ScanCounters,
    results_written: u64,
) {
    let Some(callback) = progress else {
        return;
    };

    callback(IntegrityCheckProgress {
        phase,
        current_path,
        summary,
        files_seen: counters.total_files,
        dirs_seen: counters.total_dirs,
        bytes_seen: counters.total_bytes,
        files_hashed: counters.hashed_files,
        results_written,
    });
}
