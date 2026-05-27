use crate::error::{Error, Result};
use std::path::{Component, Path, PathBuf};

/// Root-relative path used as `FSDoctor`'s filesystem identity.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RelativePath(String);

impl RelativePath {
    /// Creates a relative path from a path known to be under `root`.
    ///
    /// # Errors
    ///
    /// Returns an error if `path` is outside `root`, contains unsupported
    /// components, or cannot be represented as UTF-8.
    pub fn from_path_under_root(root: &Path, path: &Path) -> Result<Self> {
        let relative = path
            .strip_prefix(root)
            .map_err(|_error| Error::PathOutsideRoot {
                root: root.to_path_buf(),
                path: path.to_path_buf(),
            })?;

        Self::from_relative_path(relative)
    }

    /// Creates a relative path from a relative native path.
    ///
    /// # Errors
    ///
    /// Returns an error if the path contains unsupported components or cannot
    /// be represented as UTF-8.
    pub fn from_relative_path(path: &Path) -> Result<Self> {
        let mut parts = Vec::new();

        for component in path.components() {
            match component {
                Component::Normal(part) => {
                    let text = part.to_str().ok_or_else(|| Error::UnsupportedPath {
                        path: path.to_path_buf(),
                    })?;

                    parts.push(text);
                }
                Component::CurDir => {}
                Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                    return Err(Error::UnsupportedPath {
                        path: path.to_path_buf(),
                    });
                }
            }
        }

        if parts.is_empty() {
            return Err(Error::UnsupportedPath {
                path: path.to_path_buf(),
            });
        }

        Ok(Self(parts.join("/")))
    }

    /// Returns database/display representation.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

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
