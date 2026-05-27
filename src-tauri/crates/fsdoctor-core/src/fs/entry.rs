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
