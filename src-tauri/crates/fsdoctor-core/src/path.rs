use crate::error::{Error, Result};
use std::path::{Path, PathBuf};

/// Converts a path to a database text value.
///
/// `FSDoctor` stores project root paths as text. For the MVP, paths must be
/// valid UTF-8. Unsupported paths are rejected explicitly instead of being
/// lossy-converted.
///
/// # Errors
/// Returns [`Error::UnsupportedPath`] if it fails to convert a native path to string.
pub fn path_to_db_text(path: &Path) -> Result<String> {
    let text = path.to_str().ok_or_else(|| Error::UnsupportedPath {
        path: path.to_path_buf(),
    })?;

    Ok(text.to_owned())
}

/// Converts a bd path string back to a native path.
#[must_use]
pub fn db_text_to_path(text: &str) -> PathBuf {
    PathBuf::from(text)
}
