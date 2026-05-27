use std::fs::{self, FileType};
use std::path::Path;

use crate::error::{Error, Result};
use crate::{collect_metadata, FsEntry, FsEntryKind, FsEntryStatus, RelativePath, SkipReason};

/// Scanner options.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ScanOptions {
    /// Whether symbolic links should be followed.
    ///
    /// The default is `false`.
    pub follow_symlinks: bool,

    /// Whether Windows reparse points should be followed.
    ///
    /// The default is `false`.
    pub follow_reparse_points: bool,
}

/// Summary produced by filesystem scan.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanFlow {
    /// Continue scanning.
    Continue,

    /// Stop scanning without treating it as an error.
    Stop,
}

/// Summary produced by a filesystem scan.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ScanSummary {
    /// Number of regular files encountered.
    pub files: u64,

    /// Number of directories encountered.
    pub directories: u64,

    /// Number of symlinks encountered.
    pub symlinks: u64,

    /// Number of other entries encountered.
    pub other: u64,

    /// Number of entries skipped by policy.
    pub skipped: u64,

    /// Number of unreadable entries.
    pub unreadable: u64,
}

/// Recursively scans a filesystem tree.
///
/// The scan root itself is validated but not emitted as an entry.
///
/// # Errors
///
/// Returns an error if the root is invalid or if the callback returns an error.
pub fn scan_tree(
    root: &Path,
    options: ScanOptions,
    mut on_entry: impl FnMut(FsEntry) -> Result<ScanFlow>,
) -> Result<ScanSummary> {
    let root_metadata = fs::symlink_metadata(root).map_err(|source| Error::FileSystem {
        path: root.to_path_buf(),
        source,
    })?;

    if !root_metadata.is_dir() {
        return Err(Error::InvalidScanRoot {
            path: root.to_path_buf(),
        });
    }

    let mut summary = ScanSummary::default();

    scan_directory(root, root, options, &mut summary, &mut on_entry)?;

    Ok(summary)
}

/// Recursively scans a directory.
fn scan_directory(
    root: &Path,
    directory: &Path,
    options: ScanOptions,
    summary: &mut ScanSummary,
    on_entry: &mut impl FnMut(FsEntry) -> Result<ScanFlow>,
) -> Result<ScanFlow> {
    let entries = match fs::read_dir(directory) {
        Ok(entries) => entries,
        Err(source) => {
            let relative_path = RelativePath::from_path_under_root(root, directory)?;

            let entry = FsEntry {
                relative_path,
                absolute_path: directory.to_path_buf(),
                kind: FsEntryKind::Directory,
                metadata: None,
                status: FsEntryStatus::Unreadable {
                    message: source.to_string(),
                },
            };

            increment(&mut summary.unreadable);

            return on_entry(entry);
        }
    };

    for entry_result in entries {
        let dir_entry = match entry_result {
            Ok(entry) => entry,
            Err(source) => {
                let relative_path = RelativePath::from_path_under_root(root, directory)?;

                let entry = FsEntry {
                    relative_path,
                    absolute_path: directory.to_path_buf(),
                    kind: FsEntryKind::Directory,
                    metadata: None,
                    status: FsEntryStatus::Unreadable {
                        message: source.to_string(),
                    },
                };

                increment(&mut summary.unreadable);

                if on_entry(entry)? == ScanFlow::Stop {
                    return Ok(ScanFlow::Stop);
                }

                continue;
            }
        };

        let path = dir_entry.path();
        let scanned = scan_one_entry(root, &path, options, summary)?;

        let should_descend = should_descend(&scanned, options);

        if on_entry(scanned)? == ScanFlow::Stop {
            return Ok(ScanFlow::Stop);
        }

        if should_descend
            && scan_directory(root, &path, options, summary, on_entry)? == ScanFlow::Stop
        {
            return Ok(ScanFlow::Stop);
        }
    }

    Ok(ScanFlow::Continue)
}

/// Scans a single entry.
fn scan_one_entry(
    root: &Path,
    path: &Path,
    options: ScanOptions,
    summary: &mut ScanSummary,
) -> Result<FsEntry> {
    let relative_path = RelativePath::from_path_under_root(root, path)?;

    let metadata = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(source) => {
            increment(&mut summary.unreadable);

            return Ok(FsEntry {
                relative_path,
                absolute_path: path.to_path_buf(),
                kind: FsEntryKind::Other,
                metadata: None,
                status: FsEntryStatus::Unreadable {
                    message: source.to_string(),
                },
            });
        }
    };

    let file_type = metadata.file_type();
    let kind = classify_file_type(file_type);
    let collected = collect_metadata(path, kind, &metadata)?;
    let status = entry_status(kind, collected.is_reparse_point, options);

    update_summary(summary, kind, &status);

    Ok(FsEntry {
        relative_path,
        absolute_path: path.to_path_buf(),
        kind,
        metadata: Some(collected),
        status,
    })
}

/// Classifies a native file type.
fn classify_file_type(file_type: FileType) -> FsEntryKind {
    if file_type.is_symlink() {
        FsEntryKind::Symlink
    } else if !file_type.is_dir() {
        FsEntryKind::File
    } else if file_type.is_dir() {
        FsEntryKind::Directory
    } else {
        FsEntryKind::Other
    }
}

/// Computes scanner status for an entry.
fn entry_status(kind: FsEntryKind, is_reparse_point: bool, options: ScanOptions) -> FsEntryStatus {
    if kind == FsEntryKind::Symlink && !options.follow_symlinks {
        return FsEntryStatus::Skipped {
            reason: SkipReason::Symlink,
        };
    }

    if is_reparse_point && !options.follow_reparse_points {
        return FsEntryStatus::Skipped {
            reason: SkipReason::ReparsePoint,
        };
    }

    if kind == FsEntryKind::Other {
        return FsEntryStatus::Skipped {
            reason: SkipReason::UnsupportedFileType,
        };
    }

    FsEntryStatus::Accessible
}

/// Returns whether the scanner should descend into this entry.
fn should_descend(entry: &FsEntry, _options: ScanOptions) -> bool {
    entry.kind == FsEntryKind::Directory && entry.status == FsEntryStatus::Accessible
}

/// Updates summary counters.
const fn update_summary(summary: &mut ScanSummary, kind: FsEntryKind, status: &FsEntryStatus) {
    match kind {
        FsEntryKind::File => increment(&mut summary.files),
        FsEntryKind::Directory => increment(&mut summary.directories),
        FsEntryKind::Symlink => increment(&mut summary.symlinks),
        FsEntryKind::Other => increment(&mut summary.other),
    }

    match *status {
        FsEntryStatus::Accessible => {}
        FsEntryStatus::Skipped { .. } => increment(&mut summary.skipped),
        FsEntryStatus::Unreadable { .. } => increment(&mut summary.unreadable),
    }
}

/// Saturating counter increment.
const fn increment(counter: &mut u64) {
    *counter = counter.saturating_add(1);
}
