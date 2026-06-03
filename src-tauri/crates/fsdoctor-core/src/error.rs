use std::path::PathBuf;

use thiserror::Error;

/// Result type used by `FSDoctor` core operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Error type used by `FSDoctor` core operations.
#[derive(Debug, Error)]
pub enum Error {
    /// A filesystem operation failed.
    #[error("filesystem operation failed for `{path}`: {source}")]
    FileSystem {
        /// Path involved in the failed operation.
        path: PathBuf,

        /// Underlying I/O error.
        #[source]
        source: std::io::Error,
    },

    /// A database operation failed.
    #[error("database operation failed: {0}")]
    Database(#[from] sqlx::Error),

    /// A database migration failed.
    #[error("database migration failed: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),

    /// The selected database is not an `FSDoctor` project database.
    #[error("the selected database is not an FSDoctor project database")]
    InvalidProjectDatabase,

    /// The selected database uses an unsupported format version.
    #[error("unsupported FSDoctor database format version `{actual}`, expected `{expected}`")]
    UnsupportedFormatVersion {
        /// Expected database format version.
        expected: i64,

        /// Actual database format.
        actual: i64,
    },

    /// Selected DB contians more than one project.
    #[error("the selected database contains an unsupported number of projects: {count}")]
    UnsupportedProjectCount {
        /// Number of projects found in the DB.
        count: i64,
    },

    /// The provided path could not be represented as a supported `FSDoctor` path.
    #[error("unsupported path `{path}`")]
    UnsupportedPath {
        /// Path that could not be handled.
        path: std::path::PathBuf,
    },

    /// The selected scan root is not a directory.
    #[error("scan root is not a directory: `{path}`")]
    InvalidScanRoot {
        /// Path that was selected as a scan root.
        path: PathBuf,
    },

    /// A path was expected to be inside the scan root, but was not.
    #[error("path `{path}` is outside the scan root `{root}`")]
    PathOutsideRoot {
        /// Scan root.
        root: PathBuf,

        /// Path that was outside the scan root.
        path: PathBuf,
    },

    /// A filesystem timestamp could not be represented by `FSDoctor`.
    #[error("filesystem timestamp for `{path}` is outside the supported range")]
    TimestampOutOfRange {
        /// Path whose timestamp could not be represented.
        path: PathBuf,
    },

    /// Invalid hash chunk size was configured.
    #[error("invalid hash chunk size")]
    InvalidHashChunkSize,

    /// Hashing was cancelled by the caller.
    #[error("hashing was cancelled")]
    HashingCancelled,

    /// The selected path is not a regular file and cannot be hashed.
    #[error("provided path is not a regular file and cannot be hashed: `{path}`")]
    NotRegularFile {
        /// Path that was requested for hashing.
        path: PathBuf,
    },

    /// A file had been changed while it was being hashed.
    #[error("file changed while being hashed: `{path}`")]
    ChangedDuringHash {
        /// Path to file changed during hashing.
        path: PathBuf,
    },
}
