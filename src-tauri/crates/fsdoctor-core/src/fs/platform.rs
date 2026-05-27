use std::fs::Metadata;

/// Returns whether metadata represents a Windows reparse point.
#[cfg(windows)]
#[must_use]
pub fn is_reparse_point(metadata: &Metadata) -> bool {
    use std::os::windows::fs::MetadataExt;

    /// Windows `FILE_ATTRIBUTE_REPARSE_POINT`.
    const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0000_0400;

    metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0
}

/// Returns whether metadata represents a Windows reparse point.
#[cfg(not(windows))]
#[must_use]
pub const fn is_reparse_point(_metadata: &Metadata) -> bool {
    false
}
