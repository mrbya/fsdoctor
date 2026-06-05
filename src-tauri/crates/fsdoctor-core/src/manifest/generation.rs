use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use tokio::{
    sync::mpsc,
    task::{self, JoinHandle},
};

use crate::{
    db::{manifest::ManifestEntryRecord, scan::ScanCounters},
    hash_file,
    manifest::model::{ManifestGenerationOptions, ManifestGenerationReport},
    manifest::progress::{ManifestGenerationPhase, ManifestGenerationProgress},
    scan_tree, CancelToken, Error, FsEntry, FsEntryKind, FsEntryStatus, HashOptions, HashOutcome,
    ManifestEntryStatus, ProjectDb, ProjectId, Result, ScanFlow, ScanId, ScanKind, ScanOptions,
    ScanStatus,
};

/// Number of manifest records buffered between producer and DB writer.
const MANIFEST_RECORD_CHANNEL_CAPACITY: usize = 1024;

/// Progress callback type used by the manifest-generation engine.
type ProgressCallback = Arc<dyn Fn(ManifestGenerationProgress) + Send + Sync + 'static>;

/// Shared manifest progress state.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct ManifestProgressState {
    /// Latest scanned counters.
    counters: ScanCounters,

    /// Latest current path.
    current_path: Option<String>,

    /// Entries written to the database.
    results_written: u64,
}

/// Internal producer result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ProducerReport {
    /// Scan counters collected by the producer.
    counters: ScanCounters,

    /// Stopped because of a cancellation request?
    cancelled: bool,
}

/// Result of converting one filesystem entry into a manifest record.
#[derive(Debug, Clone, PartialEq, Eq)]
enum RecordProduction {
    /// A manifest record was produced.
    Record(ManifestEntryRecord),

    /// Cancellation was observed.
    Cancelled,
}

/// Generates and persists a file-tree manifest for the project database.
///
/// # Errors
///
/// Returns an error if scan creation, filesystem scanning, hashing, worker
/// execution, or database persistence fails.
pub async fn generate_manifest(
    db: &ProjectDb,
    options: ManifestGenerationOptions,
    cancel_token: &CancelToken,
) -> Result<ManifestGenerationReport> {
    generate_manifest_inner(db, options, cancel_token, None).await
}

/// Generates and persists a file-tree manifest while reporting progress.
///
/// # Errors
///
/// Returns an error if scan creation, filesystem scanning, hashing, worker
/// execution, or database persistence fails.
pub async fn generate_manifest_with_progress(
    db: &ProjectDb,
    options: ManifestGenerationOptions,
    cancel_token: &CancelToken,
    on_progress: impl Fn(ManifestGenerationProgress) + Send + Sync + 'static,
) -> Result<ManifestGenerationReport> {
    generate_manifest_inner(db, options, cancel_token, Some(Arc::new(on_progress))).await
}

/// Internal manifest-generation implementation.
async fn generate_manifest_inner(
    db: &ProjectDb,
    options: ManifestGenerationOptions,
    cancel_token: &CancelToken,
    progress: Option<ProgressCallback>,
) -> Result<ManifestGenerationReport> {
    if options.db_batch_size == 0 {
        return Err(Error::InvalidManifestBatchSize);
    }

    let project = db.project().await?;
    let scan_id = db
        .create_scan(project.id, ScanKind::ManifestGeneration)
        .await?;

    let (sender, receiver) = mpsc::channel::<ManifestEntryRecord>(MANIFEST_RECORD_CHANNEL_CAPACITY);

    let root_path = project.root_path.clone();
    let project_id = project.id;
    let producer_cancel_token = cancel_token.clone();
    let progress_state = Arc::new(Mutex::new(ManifestProgressState::default()));
    let producer_progress = progress.clone();
    let producer_progress_state = Arc::clone(&progress_state);

    let producer = task::spawn_blocking(move || {
        produce_manifest_records(
            project_id,
            scan_id,
            &root_path,
            &sender,
            &producer_cancel_token,
            producer_progress.as_ref(),
            &producer_progress_state,
        )
    });

    let writer_result = consume_manifest_record(
        db,
        receiver,
        options.db_batch_size,
        progress.clone(),
        Arc::clone(&progress_state),
    )
    .await;

    if writer_result.is_err() {
        cancel_token.cancel();
    }

    let producer_result = await_producer(producer).await;

    finish_generation(
        db,
        scan_id,
        producer_result,
        writer_result,
        progress.as_ref(),
        &progress_state,
    )
    .await
}

/// Awaits the blocking producer task.
async fn await_producer(producer: JoinHandle<Result<ProducerReport>>) -> Result<ProducerReport> {
    producer
        .await
        .map_err(|source| Error::ManifestWorkerJoin { source })?
}

/// Finishes scan lifecycle and returns final public report.
async fn finish_generation(
    db: &ProjectDb,
    scan_id: ScanId,
    producer_result: Result<ProducerReport>,
    writer_result: Result<()>,
    progress: Option<&ProgressCallback>,
    progress_state: &Arc<Mutex<ManifestProgressState>>,
) -> Result<ManifestGenerationReport> {
    match (producer_result, writer_result) {
        (Ok(producer_report), Ok(())) => {
            let status = if producer_report.cancelled {
                ScanStatus::Cancelled
            } else {
                ScanStatus::Completed
            };

            emit_progress(
                progress,
                progress_state,
                ManifestGenerationPhase::Finishing,
                None,
                producer_report.counters,
                None,
            );

            db.finish_scan(scan_id, status, producer_report.counters, None)
                .await?;

            Ok(ManifestGenerationReport {
                scan_id,
                counters: producer_report.counters,
                cancelled: producer_report.cancelled,
            })
        }

        (Ok(producer_report), Err(error)) => {
            let message = error.to_string();

            db.finish_scan(
                scan_id,
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
                scan_id,
                ScanStatus::Failed,
                ScanCounters::default(),
                Some(&message),
            )
            .await?;

            Err(error)
        }
    }
}

/// Produces manifest records by walking and hashing the filesystem tree.
///
/// This function is intended to run in a blocking worker thread.
fn produce_manifest_records(
    project_id: ProjectId,
    scan_id: ScanId,
    root_path: &Path,
    sender: &mpsc::Sender<ManifestEntryRecord>,
    cancel_token: &CancelToken,
    progress: Option<&ProgressCallback>,
    progress_state: &Arc<Mutex<ManifestProgressState>>,
) -> Result<ProducerReport> {
    let mut counters = ScanCounters::default();
    let mut cancelled = false;

    scan_tree(root_path, ScanOptions::default(), |entry| {
        if cancel_token.is_cancelled() {
            cancelled = true;
            return Ok(ScanFlow::Stop);
        }

        match manifest_record_from_entry(project_id, scan_id, entry, &mut counters, cancel_token)? {
            RecordProduction::Record(record) => {
                let current_path = record.relative_path.as_str().to_owned();

                emit_progress(
                    progress,
                    progress_state,
                    ManifestGenerationPhase::WalkingAndHashing,
                    Some(current_path),
                    counters,
                    None,
                );

                if sender.blocking_send(record).is_err() {
                    cancelled = cancel_token.is_cancelled();
                    return Ok(ScanFlow::Stop);
                }

                Ok(ScanFlow::Continue)
            }
            RecordProduction::Cancelled => {
                cancelled = true;
                Ok(ScanFlow::Stop)
            }
        }
    })?;

    Ok(ProducerReport {
        counters,
        cancelled: cancelled || cancel_token.is_cancelled(),
    })
}

/// Consumes manifest records and writes them to project database in batches.
async fn consume_manifest_record(
    db: &ProjectDb,
    mut receiver: mpsc::Receiver<ManifestEntryRecord>,
    batch_size: usize,
    progress: Option<ProgressCallback>,
    progress_state: Arc<Mutex<ManifestProgressState>>,
) -> Result<()> {
    let mut batch = Vec::with_capacity(batch_size);
    let mut results_written = 0_u64;

    while let Some(record) = receiver.recv().await {
        batch.push(record);

        if batch.len() >= batch_size {
            db.upsert_manifest_entries(&batch).await?;
            results_written = results_written.saturating_add(
                u64::try_from(batch.len()).map_err(|_error| Error::NumericOverflow)?,
            );
            batch.clear();

            emit_progress(
                progress.as_ref(),
                &progress_state,
                ManifestGenerationPhase::Writing,
                None,
                ScanCounters::default(),
                Some(results_written),
            );
        }
    }

    if !batch.is_empty() {
        db.upsert_manifest_entries(&batch).await?;
        results_written = results_written
            .saturating_add(u64::try_from(batch.len()).map_err(|_error| Error::NumericOverflow)?);
    }

    emit_progress(
        progress.as_ref(),
        &progress_state,
        ManifestGenerationPhase::Writing,
        None,
        ScanCounters::default(),
        Some(results_written),
    );

    Ok(())
}

/// Converts a scanned filesystem entry into a manifest record.
fn manifest_record_from_entry(
    project_id: ProjectId,
    scan_id: ScanId,
    entry: FsEntry,
    counters: &mut ScanCounters,
    cancel_token: &CancelToken,
) -> Result<RecordProduction> {
    update_seen_counters(counters, &entry);

    match entry.status {
        FsEntryStatus::Accessible => match entry.kind {
            FsEntryKind::File => {
                hash_file_entry(project_id, scan_id, entry, counters, cancel_token)
            }
            FsEntryKind::Directory => Ok(RecordProduction::Record(record_non_file(
                project_id,
                scan_id,
                entry,
                ManifestEntryStatus::Recorded,
                None,
            ))),
            FsEntryKind::Symlink | FsEntryKind::Other => {
                Ok(RecordProduction::Record(record_non_file(
                    project_id,
                    scan_id,
                    entry,
                    ManifestEntryStatus::Skipped,
                    Some(String::from("entry type skipped by policy")),
                )))
            }
        },
        FsEntryStatus::Skipped { reason } => Ok(RecordProduction::Record(record_non_file(
            project_id,
            scan_id,
            entry,
            ManifestEntryStatus::Skipped,
            Some(reason.to_str().to_owned()),
        ))),
        FsEntryStatus::Unreadable { ref message } => {
            counters.unreadable_entries = counters.unreadable_entries.saturating_add(1);

            Ok(RecordProduction::Record(record_non_file(
                project_id,
                scan_id,
                entry.clone(),
                ManifestEntryStatus::Unreadable,
                Some(message.to_owned()),
            )))
        }
    }
}

/// Hashes an accessible regular file and generates its manifest record.
fn hash_file_entry(
    project_id: ProjectId,
    scan_id: ScanId,
    entry: FsEntry,
    counters: &mut ScanCounters,
    cancel_token: &CancelToken,
) -> Result<RecordProduction> {
    let readonly = entry
        .metadata
        .as_ref()
        .is_some_and(|metadata| metadata.readonly);

    match hash_file(&entry.absolute_path, HashOptions::default(), cancel_token) {
        Ok(HashOutcome::Hashed(hashed)) => {
            counters.hashed_files = counters.hashed_files.saturating_add(1);

            Ok(RecordProduction::Record(ManifestEntryRecord {
                project_id,
                scan_id,
                relative_path: entry.relative_path,
                entry_kind: entry.kind,
                size_bytes: Some(hashed.fingerprint.size_bytes),
                mtime_ns: hashed.fingerprint.modified_time_ns,
                readonly,
                hash_algo: Some(hashed.digest.algorithm.as_str()),
                hash: Some(hashed.digest.bytes),
                status: ManifestEntryStatus::Hashed,
                error_message: None,
            }))
        }
        Ok(HashOutcome::ChangedDuringHash { after, .. }) => {
            counters.changed_during_scan = counters.changed_during_scan.saturating_add(1);

            Ok(RecordProduction::Record(ManifestEntryRecord {
                project_id,
                scan_id,
                relative_path: entry.relative_path,
                entry_kind: entry.kind,
                size_bytes: Some(after.size_bytes),
                mtime_ns: after.modified_time_ns,
                readonly,
                hash_algo: None,
                hash: None,
                status: ManifestEntryStatus::ChangedDuringScan,
                error_message: Some(String::from("file changed while being hashed")),
            }))
        }
        Err(Error::HashingCancelled) => Ok(RecordProduction::Cancelled),
        Err(Error::FileSystem { source, .. }) => {
            counters.unreadable_entries = counters.unreadable_entries.saturating_add(1);

            Ok(RecordProduction::Record(ManifestEntryRecord {
                project_id,
                scan_id,
                relative_path: entry.relative_path,
                entry_kind: entry.kind,
                size_bytes: entry
                    .metadata
                    .as_ref()
                    .and_then(|metadata| metadata.size_bytes),
                mtime_ns: entry
                    .metadata
                    .as_ref()
                    .and_then(|metadata| metadata.modified_time_ns),
                readonly,
                hash_algo: None,
                hash: None,
                status: ManifestEntryStatus::Unreadable,
                error_message: Some(source.to_string()),
            }))
        }
        Err(error) => Err(error),
    }
}

/// Record a non-file or non-hashed entry.
fn record_non_file(
    project_id: ProjectId,
    scan_id: ScanId,
    entry: FsEntry,
    status: ManifestEntryStatus,
    error_message: Option<String>,
) -> ManifestEntryRecord {
    let metadata = entry.metadata.as_ref();

    ManifestEntryRecord {
        project_id,
        scan_id,
        relative_path: entry.relative_path,
        entry_kind: entry.kind,
        size_bytes: metadata.and_then(|value| value.size_bytes),
        mtime_ns: metadata.and_then(|value| value.modified_time_ns),
        readonly: metadata.is_some_and(|value| value.readonly),
        hash_algo: None,
        hash: None,
        status,
        error_message,
    }
}

/// Emits manifest progress if a callback exists.
fn emit_progress(
    progress: Option<&ProgressCallback>,
    progress_state: &Arc<Mutex<ManifestProgressState>>,
    phase: ManifestGenerationPhase,
    current_path: Option<String>,
    counters: ScanCounters,
    results_written: Option<u64>,
) {
    let Some(callback) = progress else {
        return;
    };

    let snapshot = {
        let mut state = progress_state
            .lock()
            .expect("manifest progress state lock should not be poisoned");

        if counters != ScanCounters::default() {
            state.counters = counters;
        }

        if current_path.is_some() || phase != ManifestGenerationPhase::Writing {
            state.current_path = current_path;
        }

        if let Some(results_written) = results_written {
            state.results_written = results_written;
        }

        ManifestGenerationProgress {
            phase,
            current_path: state.current_path.clone(),
            files_seen: state.counters.total_files,
            dirs_seen: state.counters.total_dirs,
            bytes_seen: state.counters.total_bytes,
            files_hashed: state.counters.hashed_files,
            bytes_hashed: state.counters.total_bytes,
            unreadable_entries: state.counters.unreadable_entries,
            changed_during_scan: state.counters.changed_during_scan,
            results_written: state.results_written,
        }
    };

    callback(snapshot);
}

/// Updateds countrs for an entry observed by the scanner.
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
