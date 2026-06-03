use std::{
    fs::Metadata,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    error::{Error, Result},
    fs::platform::is_reparse_point,
    FsEntryKind, FsMetadata,
};

/// Converts native filesystem metadata into `FSDoctor` metadata.
///
/// # Errors
///
/// Returns an error if a timestamp is outside the supported range.
pub fn collect_metadata(path: &Path, kind: FsEntryKind, metadata: &Metadata) -> Result<FsMetadata> {
    Ok(FsMetadata {
        size_bytes: size_for_kind(kind, metadata),
        modified_time_ns: modified_time_ns(path, metadata)?,
        readonly: metadata.permissions().readonly(),
        is_reparse_point: is_reparse_point(metadata),
    })
}

/// Retrieves Unix epoch relative ns modification time from file metadata.
///
/// # Errors
///
/// Returns an error if underlying system time retrieval fails.
pub fn metadata_modified_time_ns(path: &Path, metadata: &Metadata) -> Result<Option<i128>> {
    let Ok(modified) = metadata.modified() else {
        return Ok(None);
    };

    system_time_to_unix_ns(path, modified).map(Some)
}

/// Returns the file size for regular files.
fn size_for_kind(kind: FsEntryKind, metadata: &Metadata) -> Option<u64> {
    match kind {
        FsEntryKind::File => Some(metadata.len()),
        _ => None,
    }
}

/// Converts modification time to signed nanoseconds relative to Unix epoch.
fn modified_time_ns(path: &Path, metadata: &Metadata) -> Result<Option<i128>> {
    let Ok(modified) = metadata.modified() else {
        return Ok(None);
    };

    system_time_to_unix_ns(path, modified).map(Some)
}

/// Converts a [`SystemTime`] to signed nanoseconds relative to Unix epoch.
fn system_time_to_unix_ns(path: &Path, time: SystemTime) -> Result<i128> {
    match time.duration_since(UNIX_EPOCH) {
        Ok(duration) => {
            i128::try_from(duration.as_nanos()).map_err(|_error| Error::TimestampOutOfRange {
                path: path.to_path_buf(),
            })
        }
        Err(error) => {
            let nanos = i128::try_from(error.duration().as_nanos()).map_err(|_error| {
                Error::TimestampOutOfRange {
                    path: path.to_path_buf(),
                }
            })?;
            nanos
                .checked_neg()
                .ok_or_else(|| Error::TimestampOutOfRange {
                    path: path.to_path_buf(),
                })
        }
    }
}
