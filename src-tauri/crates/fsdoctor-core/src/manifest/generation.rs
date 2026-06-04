use tokio::sync::mpsc;

use crate::{
    db::{manifest::ManifestEntryRecord, scan::ScanCounters},
    hash_file, CancelToken, Error, FsEntry, FsEntryKind, FsEntryStatus, HashOptions, HashOutcome,
    ManifestEntryStatus, ProjectDb, ProjectId, Result, ScanId,
};

/// Number of manifest records buffered between producer and DB writer.
const MANIFEST_RECORD_CHANNEL_CAPACITY: usize = 1024;

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

/// Consumes manifest records and writes them to project database in batches.
async fn consume_manifest_record(
    db: &ProjectDb,
    mut receiver: mpsc::Receiver<ManifestEntryRecord>,
    batch_size: usize,
) -> Result<()> {
    let mut batch = Vec::with_capacity(batch_size);

    while let Some(record) = receiver.recv().await {
        batch.push(record);

        if batch.len() >= batch_size {
            db.upsert_manifest_entries(&batch).await?;
            batch.clear();
        }
    }

    if !batch.is_empty() {
        db.upsert_manifest_entries(&batch).await?;
    }

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
                size_bytes: entry.metadata.and_then(|metadata| metadata.size_bytes),
                mtime_ns: entry
                    .metadata
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
