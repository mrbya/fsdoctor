use std::{
    fs::{symlink_metadata, File, Metadata},
    io::{BufReader, Read},
    path::{Path, PathBuf},
};

use blake3::Hasher;

use crate::{
    error::{Error, Result},
    hash::cancel::CancelToken,
};
use crate::{fs::metadata::metadata_modified_time_ns, hash::digest::FileDigest};

/// Default hashing buffer size.
pub const DEFAULT_HASH_CHUNK_SIZE: usize = 1024 * 1024;

/// Hashing options.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HashOptions {
    /// Read buffer size.
    pub chunk_size: usize,
}

impl Default for HashOptions {
    fn default() -> Self {
        Self {
            chunk_size: DEFAULT_HASH_CHUNK_SIZE,
        }
    }
}

/// Stable file fingerprint used to detect mutation during hashing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileFingerprint {
    /// File size in bytes.
    pub size_bytes: u64,

    /// Modification time as signed nanoseconds relative to Unix epoch.
    pub modified_time_ns: Option<i128>,
}

/// Successful hash result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HashedFile {
    /// Path to the hashed file.
    pub path: PathBuf,

    /// File fingerprint observed after hashing.
    pub fingerprint: FileFingerprint,

    /// File digest.
    pub digest: FileDigest,
}

/// Outcome of hashing a file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HashOutcome {
    /// File was successfully hashed.
    Hashed(HashedFile),

    /// File changed during hashing.
    ChangedDuringHash {
        /// Path to file.
        path: PathBuf,

        /// Fingerprint before hashing.
        before: FileFingerprint,

        /// Fingerprint after hashing.
        after: FileFingerprint,
    },
}

/// Hashes a regular file with BLAKE3.
///
/// # Errors
///
/// Returns an error if the file cannot be opened/read, is not a regular file,
/// or cancellation is requested.
pub fn hash_file(
    path: &Path,
    options: HashOptions,
    cancel_token: &CancelToken,
) -> Result<HashOutcome> {
    let before_metadata = symlink_metadata(path).map_err(|source| Error::FileSystem {
        path: path.to_path_buf(),
        source,
    })?;

    if !before_metadata.file_type().is_file() {
        return Err(Error::NotRegularFile {
            path: path.to_path_buf(),
        });
    }

    let before = fingerprint_from_metadata(path, &before_metadata)?;

    let file = File::open(path).map_err(|source| Error::FileSystem {
        path: path.to_path_buf(),
        source,
    })?;

    let digest = hash_open_file(path, file, options, cancel_token)?;

    let after_metadata = symlink_metadata(path).map_err(|source| Error::FileSystem {
        path: path.to_path_buf(),
        source,
    })?;

    let after = fingerprint_from_metadata(path, &after_metadata)?;

    if before != after {
        return Ok(HashOutcome::ChangedDuringHash {
            path: path.to_path_buf(),
            before,
            after,
        });
    }

    Ok(HashOutcome::Hashed(HashedFile {
        path: path.to_path_buf(),
        fingerprint: after,
        digest,
    }))
}

/// Hashes an already opened file.
fn hash_open_file(
    path: &Path,
    file: File,
    options: HashOptions,
    cancel_token: &CancelToken,
) -> Result<FileDigest> {
    if options.chunk_size == 0 {
        return Err(Error::InvalidHashChunkSize);
    }

    let mut reader = BufReader::with_capacity(options.chunk_size, file);
    let mut buffer = vec![0_u8; options.chunk_size];
    let mut hasher = Hasher::new();

    loop {
        if cancel_token.is_cancelled() {
            return Err(Error::HashingCancelled);
        }

        let bytes_read = reader
            .read(&mut buffer)
            .map_err(|source| Error::FileSystem {
                path: path.to_path_buf(),
                source,
            })?;

        if bytes_read == 0 {
            break;
        }

        let chunk = buffer
            .get(..bytes_read)
            .ok_or(Error::InvalidHashChunkSize)?;

        hasher.update(chunk);
    }

    let digest = hasher.finalize();

    Ok(FileDigest::blake3(*digest.as_bytes()))
}

/// Converts file metadata to mutation-detection fingerprint.
fn fingerprint_from_metadata(path: &Path, metadata: &Metadata) -> Result<FileFingerprint> {
    Ok(FileFingerprint {
        size_bytes: metadata.len(),
        modified_time_ns: metadata_modified_time_ns(path, metadata)?,
    })
}
