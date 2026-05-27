use std::path::PathBuf;

use time::OffsetDateTime;

/// Current `FSDoctor` project database format version.
pub const CURRENT_PROJECT_FORMAT_VERSION: i64 = 1;

/// Stored project metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Project {
    /// Database-local project identifier.
    pub id: ProjectId,

    /// User-facing project name.
    pub name: String,

    /// Absolute backup root path selected for this project.
    pub root_path: PathBuf,

    /// Project creation timestamp.
    pub created_at: OffsetDateTime,

    /// Last metadata update timestamp.
    pub updated_at: OffsetDateTime,

    /// `FSDoctor` manifestdb format version.
    pub format_version: i64,
}

/// Database-local project identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProjectId(i64);

impl ProjectId {
    /// Creates a project identifier from a raw database id integer.
    #[must_use]
    pub const fn from_raw(raw: i64) -> Self {
        Self(raw)
    }

    /// Returns the raw db id.
    #[must_use]
    pub const fn raw(self) -> i64 {
        self.0
    }
}

/// Request used to create a new `FSDoctor` project database.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateProjectRequest {
    /// Path, where the `.fsdoctor.sqlite` file should be created.
    pub db_path: PathBuf,

    /// User-facing project name.
    pub name: String,

    /// Backup root path associated with this project.
    pub root_path: PathBuf,
}

/// Request used to open an exsiting `FSDoctor` project database.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenProjectRequest {
    /// Path to the existing `.fsdoctor.sqlite` db file.
    pub db_path: PathBuf,
}
