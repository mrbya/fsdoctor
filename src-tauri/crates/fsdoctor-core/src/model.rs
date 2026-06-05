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

/// Database-local scan identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScanId(i64);

impl ScanId {
    /// Creates a scan id from a raw database id.
    #[must_use]
    pub const fn from_raw(raw: i64) -> Self {
        Self(raw)
    }

    /// Returns the raw database id.
    #[must_use]
    pub const fn raw(self) -> i64 {
        self.0
    }
}

/// Scan kind stored in the database.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanKind {
    /// Manifest generation scan.
    ManifestGeneration,

    /// Integrity check scan.
    IntegrityCheck,
}

impl ScanKind {
    /// Stable DB string.
    #[must_use]
    pub const fn as_db_str(self) -> &'static str {
        match self {
            Self::ManifestGeneration => "manifest_generation",
            Self::IntegrityCheck => "integrity_check",
        }
    }
}

/// Scan lifecycle status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanStatus {
    /// Scan is currently running.
    Running,

    /// Scan completed successfully.
    Completed,

    /// Scan was canceled.
    Cancelled,

    /// Scan failed.
    Failed,
}

impl ScanStatus {
    /// Stable DB string.
    #[must_use]
    pub const fn as_db_str(self) -> &'static str {
        match self {
            Self::Running => "running",
            Self::Completed => "completed",
            Self::Cancelled => "cancelled",
            Self::Failed => "failed",
        }
    }
}

/// Manifest entry status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManifestEntryStatus {
    /// File was hashed successfully.
    Hashed,

    /// Non-file entry was recorded successfully.
    Recorded,

    /// Entry was skipped by policy.
    Skipped,

    /// Entry could not be read.
    Unreadable,

    /// File changed while being hashed.
    ChangedDuringScan,
}

impl ManifestEntryStatus {
    /// Stable DB string.
    #[must_use]
    pub const fn as_db_str(self) -> &'static str {
        match self {
            Self::Hashed => "hashed",
            Self::Recorded => "recorded",
            Self::Skipped => "skipped",
            Self::Unreadable => "unreadable",
            Self::ChangedDuringScan => "changed_during_scan",
        }
    }
}

/// Integrity check result kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckResultKind {
    /// Entry matches the manifest.
    Ok,

    /// Entry exists in the manifest but is missing from the current tree.
    Missing,

    /// Entry exists in the current tree but not in the manifest.
    New,

    /// File hash differs from the manifest.
    HashMismatch,

    /// File size differs from the manifest.
    SizeMismatch,

    /// Entry kind differs from the manifest.
    TypeChanged,

    /// Current entry could not be read.
    Unreadable,

    /// File changed while it was being checked.
    ChangedDuringCheck,

    /// Entry was skipped by policy.
    Skipped,
}

impl CheckResultKind {
    /// Stable DB string.
    #[must_use]
    pub const fn as_db_str(self) -> &'static str {
        match self {
            Self::Ok => "ok",
            Self::Missing => "missing",
            Self::New => "new",
            Self::HashMismatch => "hash_mismatch",
            Self::SizeMismatch => "size_mismatch",
            Self::TypeChanged => "type_changed",
            Self::Unreadable => "unreadable",
            Self::ChangedDuringCheck => "changed_during_check",
            Self::Skipped => "skipped",
        }
    }
}
