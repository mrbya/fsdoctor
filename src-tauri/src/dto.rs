use fsdoctor_core::ManifestGenerationReport;
use serde::{Deserialize, Serialize};

use crate::error::CommandError;

/// Request to create a new `FSDoctor` project database.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProjectRequestDto {
    /// Database path.
    pub db_path: String,

    /// User-facing project name.
    pub name: String,

    /// Backup root path.
    pub root_path: String,
}

/// Request to open an existing `FSDoctor` project database.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenProjectRequestDto {
    /// Database path.
    pub db_path: String,
}

/// Project metadata returned to the frontend.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectDto {
    /// Database-local project io.
    pub id: i64,

    /// USer-facing project name.
    pub name: String,

    /// Backup root path.
    pub root_path: String,

    /// Database format version.
    pub format_version: i64,
}

impl From<fsdoctor_core::Project> for ProjectDto {
    fn from(project: fsdoctor_core::Project) -> Self {
        Self {
            id: project.id.raw(),
            name: project.name,
            root_path: project.root_path.display().to_string(),
            format_version: project.format_version,
        }
    }
}

/// Request to start manifest generation.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartManifestGenerationRequestDto {
    /// Path to the `FSDoctor` project database.
    pub db_path: String,

    /// Optional DB batch size.
    pub db_batch_size: Option<usize>,
}

/// Request to cancel a running job.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelJobRequestDto {
    /// Job identifier returned by a start command.
    pub job_id: String,
}

/// Returned when a background job starts.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JobStartedDto {
    /// Job identifier.
    pub job_id: String,
}

/// Returned when cancellation is requested.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelJobResultDto {
    /// Job identifier.
    pub job_id: String,

    /// Whether a matching active job was found and cancellation was requested.
    pub cancellation_requested: bool,
}

/// Manifest generation event status.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ManifestGenerationEventStatusDto {
    /// Job completed successfully.
    Completed,

    /// Job completed because cancellation was requested.
    Cancelled,

    /// Job failed.
    Failed,
}

/// Manifest generation report sent to the frontend.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ManifestGenerationReportDto {
    /// Scan id.
    pub scan_id: i64,

    /// Total directories seen.
    pub total_dirs: u64,

    /// Total files seen.
    pub total_files: u64,

    /// Total symlinks seen.
    pub total_symlinks: u64,

    /// Total other entries seen.
    pub total_other: u64,

    /// Total bytes seen.
    pub total_bytes: u64,

    /// Files successfully hashed.
    pub hashed_files: u64,

    /// Unreadable entries.
    pub unreadable_entries: u64,

    /// Files changed during scan.
    pub changed_during_scan: u64,
}

impl From<ManifestGenerationReport> for ManifestGenerationReportDto {
    fn from(report: ManifestGenerationReport) -> Self {
        let counters = report.counters;

        Self {
            scan_id: report.scan_id.raw(),
            total_dirs: counters.total_dirs,
            total_files: counters.total_files,
            total_symlinks: counters.total_symlinks,
            total_other: counters.total_other,
            total_bytes: counters.total_bytes,
            hashed_files: counters.hashed_files,
            unreadable_entries: counters.unreadable_entries,
            changed_during_scan: counters.changed_during_scan,
        }
    }
}

/// Manifest generation finished event payload.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ManifestGenerationFinishedEventDto {
    /// Job identifier.
    pub job_id: String,

    /// Final job status,
    pub status: ManifestGenerationEventStatusDto,

    /// Report if the job reached core manifest generation completion.
    pub report: Option<ManifestGenerationReportDto>,

    /// Error if the job failed.
    pub error: Option<CommandError>,
}
