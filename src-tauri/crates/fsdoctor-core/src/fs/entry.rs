use std::path::PathBuf;

use crate::error::{Error, Result};
use crate::RelativePath;

/// Filesystem entry kind recognized by `FSDoctor`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FsEntryKind {
    /// Regular file.
    File,

    /// Directory.
    Directory,

    /// Symbolic link.
    Symlink,

    /// Other unsupported filesystem entry.
    Other,
}

impl FsEntryKind {
    /// Converts fs entry kind to a stable DB string.
    #[must_use]
    pub const fn as_db_str(self) -> &'static str {
        match self {
            Self::File => "file",
            Self::Directory => "directory",
            Self::Symlink => "symlink",
            Self::Other => "other",
        }
    }

    /// Converts a stable fs entry kind DB string to `FsEntryKind`.
    ///
    /// # Errors
    /// Returns [`Error::InvalidProjectDatabase`] on invalid db strings/unknown fs kinds.
    pub fn from_db_str(value: &str) -> Result<Self> {
        match value {
            "file" => Ok(Self::File),
            "directory" => Ok(Self::Directory),
            "symlink" => Ok(Self::Symlink),
            "other" => Ok(Self::Other),
            _ => Err(Error::InvalidProjectDatabase),
        }
    }
}

/// Scanner handling status for an entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FsEntryStatus {
    /// Entry was succesfully inspected.
    Accessible,

    /// Entry was intentionally skipped.
    Skipped {
        /// Reason the entry was skipped.
        reason: SkipReason,
    },

    /// Entry cound not have been read.
    Unreadable {
        /// User/developer-facing message.
        message: String,
    },
}

/// Reason an entry was skipped.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkipReason {
    /// Symbolic links are not followed by default.
    Symlink,

    /// Windows reparse points are not followed by default.
    ReparsePoint,

    /// Entry type is not supported by the scanner.
    UnsupportedFileType,
}

impl SkipReason {
    /// Converts skip reason to a human-readable string.
    #[must_use]
    pub const fn to_str(self) -> &'static str {
        match self {
            Self::Symlink => "symbolic link skipped by policy",
            Self::ReparsePoint => "reparse point skipped by policy",
            Self::UnsupportedFileType => "unsupported filesystem entry type skipped by policy",
        }
    }
}

/// Filesystem entry emitted by the scanner.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsEntry {
    /// Path relative to the scan root.
    pub relative_path: RelativePath,

    /// Absolute/native path used for filesystem access.
    pub absolute_path: PathBuf,

    /// Entry kind.
    pub kind: FsEntryKind,

    /// Metadata collected for this entry, when available.
    pub metadata: Option<FsMetadata>,

    /// Scanner status.
    pub status: FsEntryStatus,
}

/// Metadata collected for a filesystem entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsMetadata {
    /// File size in bytes.
    pub size_bytes: Option<u64>,

    /// Modification time as signed nanoseconds relative to Unix epoch.
    pub modified_time_ns: Option<i128>,

    /// Is readonly according to platform metadata?
    pub readonly: bool,

    /// Is Windows reparse point?
    pub is_reparse_point: bool,
}
