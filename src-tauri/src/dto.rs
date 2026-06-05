use fsdoctor_core::{
    IntegrityCheckPhase, IntegrityCheckProgress, IntegrityCheckReport, IntegrityCheckSummary,
    ManifestGenerationPhase, ManifestGenerationProgress, ManifestGenerationReport, Project,
};
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

impl From<Project> for ProjectDto {
    fn from(project: Project) -> Self {
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

/// Manifest generation progress phase.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ManifestGenerationPhaseDto {
    /// Walking the filesystem and hashing files.
    WalkingAndHashing,

    /// Writing manifest entries.
    Writing,

    /// Finalizing the scan.
    Finishing,
}

impl From<ManifestGenerationPhase> for ManifestGenerationPhaseDto {
    fn from(phase: ManifestGenerationPhase) -> Self {
        match phase {
            ManifestGenerationPhase::WalkingAndHashing => Self::WalkingAndHashing,
            ManifestGenerationPhase::Writing => Self::Writing,
            ManifestGenerationPhase::Finishing => Self::Finishing,
        }
    }
}

/// Manifest generation progress payload.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ManifestGenerationProgressDto {
    /// Current phase.
    pub phase: ManifestGenerationPhaseDto,

    /// Current path, if available.
    pub current_path: Option<String>,

    /// Files seen.
    pub files_seen: u64,

    /// Directories seen.
    pub dirs_seen: u64,

    /// Bytes seen.
    pub bytes_seen: u64,

    /// Files hashed.
    pub files_hashed: u64,

    /// Bytes hashed.
    pub bytes_hashed: u64,

    /// Unreadable entries.
    pub unreadable_entries: u64,

    /// Changed-during-scan entries.
    pub changed_during_scan: u64,

    /// Entries written to the database.
    pub results_written: u64,
}

impl From<ManifestGenerationProgress> for ManifestGenerationProgressDto {
    fn from(progress: ManifestGenerationProgress) -> Self {
        Self {
            phase: ManifestGenerationPhaseDto::from(progress.phase),
            current_path: progress.current_path,
            files_seen: progress.files_seen,
            dirs_seen: progress.dirs_seen,
            bytes_seen: progress.bytes_seen,
            files_hashed: progress.files_hashed,
            bytes_hashed: progress.bytes_hashed,
            unreadable_entries: progress.unreadable_entries,
            changed_during_scan: progress.changed_during_scan,
            results_written: progress.results_written,
        }
    }
}

/// Manifest generation progress event payload.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ManifestGenerationProgressEventDto {
    /// Job identifier.
    pub job_id: String,

    /// Progress snapshot.
    pub progress: ManifestGenerationProgressDto,
}

/// Request to start an integrity check.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartIntegrityCheckRequestDto {
    /// Path to the `FSDoctor` project database.
    pub db_path: String,

    /// Optional DB batch size.
    pub db_batch_size: Option<usize>,
}

/// Integrity check event status.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IntegrityCheckEventStatusDto {
    /// Job completed successfully.
    Completed,

    /// Job completed because cancellation was requested.
    Cancelled,

    /// Job failed.
    Failed,
}

/// Integrity check summary sent to the frontend.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IntegrityCheckSummaryDto {
    /// OK entries.
    pub ok: u64,

    /// Missing entries.
    pub missing: u64,

    /// New entries.
    pub new: u64,

    /// Hash mismatches.
    pub hash_mismatch: u64,

    /// Size mismatches.
    pub size_mismatch: u64,

    /// Type changes.
    pub type_changed: u64,

    /// Unreadable entries.
    pub unreadable: u64,

    /// Changed-during-check entries.
    pub changed_during_check: u64,

    /// Skipped entries.
    pub skipped: u64,
}

impl From<IntegrityCheckSummary> for IntegrityCheckSummaryDto {
    fn from(summary: IntegrityCheckSummary) -> Self {
        Self {
            ok: summary.ok,
            missing: summary.missing,
            new: summary.new,
            hash_mismatch: summary.hash_mismatch,
            size_mismatch: summary.size_mismatch,
            type_changed: summary.type_changed,
            unreadable: summary.unreadable,
            changed_during_check: summary.changed_during_check,
            skipped: summary.skipped,
        }
    }
}

/// Integrity check report sent to the frontend.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IntegrityCheckReportDto {
    /// Integrity-check scan id.
    pub scan_id: i64,

    /// Manifest-generation scan id used as baseline.
    pub manifest_scan_id: i64,

    /// Result summary.
    pub summary: IntegrityCheckSummaryDto,
}

impl From<IntegrityCheckReport> for IntegrityCheckReportDto {
    fn from(report: IntegrityCheckReport) -> Self {
        Self {
            scan_id: report.scan_id.raw(),
            manifest_scan_id: report.manifest_scan_id.raw(),
            summary: IntegrityCheckSummaryDto::from(report.summary),
        }
    }
}

/// Integrity check progress phase.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IntegrityCheckPhaseDto {
    /// Loading baseline manifest.
    LoadingManifest,

    /// Walking and comparing the current tree.
    WalkingAndChecking,

    /// Recording missing entries.
    RecordingMissingEntries,

    /// Writing results.
    Writing,

    /// Finalizing scan.
    Finishing,
}

impl From<IntegrityCheckPhase> for IntegrityCheckPhaseDto {
    fn from(phase: IntegrityCheckPhase) -> Self {
        match phase {
            IntegrityCheckPhase::LoadingManifest => Self::LoadingManifest,
            IntegrityCheckPhase::WalkingAndChecking => Self::WalkingAndChecking,
            IntegrityCheckPhase::RecordingMissingEntries => Self::RecordingMissingEntries,
            IntegrityCheckPhase::Writing => Self::Writing,
            IntegrityCheckPhase::Finishing => Self::Finishing,
        }
    }
}

/// Integrity check progress payload.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IntegrityCheckProgressDto {
    /// Current phase.
    pub phase: IntegrityCheckPhaseDto,

    /// Current path, if available.
    pub current_path: Option<String>,

    /// Current summary.
    pub summary: IntegrityCheckSummaryDto,

    /// Files seen.
    pub files_seen: u64,

    /// Directories seen.
    pub dirs_seen: u64,

    /// Bytes seen.
    pub bytes_seen: u64,

    /// Files hashed.
    pub files_hashed: u64,

    /// Results written.
    pub results_written: u64,
}

impl From<IntegrityCheckProgress> for IntegrityCheckProgressDto {
    fn from(progress: IntegrityCheckProgress) -> Self {
        Self {
            phase: IntegrityCheckPhaseDto::from(progress.phase),
            current_path: progress.current_path,
            summary: IntegrityCheckSummaryDto::from(progress.summary),
            files_seen: progress.files_seen,
            dirs_seen: progress.dirs_seen,
            bytes_seen: progress.bytes_seen,
            files_hashed: progress.files_hashed,
            results_written: progress.results_written,
        }
    }
}

/// Integrity check progress event payload.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IntegrityCheckProgressEventDto {
    /// Job identifier.
    pub job_id: String,

    /// Progress snapshot.
    pub progress: IntegrityCheckProgressDto,
}

/// Integrity check finished event payload.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IntegrityCheckFinishedEventDto {
    /// Job identifier.
    pub job_id: String,

    /// Final job status.
    pub status: IntegrityCheckEventStatusDto,

    /// Report if the job reached core integrity-check completion.
    pub report: Option<IntegrityCheckReportDto>,

    /// Error if the job failed.
    pub error: Option<CommandError>,
}
