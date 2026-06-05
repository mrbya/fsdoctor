use std::path::PathBuf;

use fsdoctor_core::{
    generate_manifest, run_integrity_check_with_progress, CancelToken, IntegrityCheckOptions,
    ManifestGenerationOptions, OpenProjectRequest, ProjectDb,
};
use tauri::{AppHandle, Emitter, Manager};

use crate::{
    dto::{
        IntegrityCheckEventStatusDto, IntegrityCheckFinishedEventDto, IntegrityCheckProgressDto,
        IntegrityCheckProgressEventDto, IntegrityCheckReportDto, ManifestGenerationEventStatusDto,
        ManifestGenerationFinishedEventDto, ManifestGenerationReportDto,
    },
    error::CommandError,
    state::AppState,
};

/// Runs manifest generation and emits the final frontend event.
pub async fn run_manifest_generation_job(
    app: AppHandle,
    job_id: String,
    db_path: PathBuf,
    db_batch_size: usize,
    cancel_token: CancelToken,
) {
    let result = run_manifest_generation(db_path, db_batch_size, &cancel_token).await;
    let payload = match result {
        Ok(report) => {
            let status = if report.cancelled {
                ManifestGenerationEventStatusDto::Cancelled
            } else {
                ManifestGenerationEventStatusDto::Completed
            };

            ManifestGenerationFinishedEventDto {
                job_id: job_id.clone(),
                status,
                report: Some(ManifestGenerationReportDto::from(report)),
                error: None,
            }
        }
        Err(error) => ManifestGenerationFinishedEventDto {
            job_id: job_id.clone(),
            status: ManifestGenerationEventStatusDto::Failed,
            report: None,
            error: Some(CommandError::from(error)),
        },
    };

    if let Err(error) = app.emit("manifest-generation-finished", payload) {
        eprintln!("failed to emit manifest generation event: {error}");
    }

    let state = app.state::<AppState>();

    if let Err(error) = state.remove_job(&job_id) {
        eprintln!("failed to remove completed manifest job: {error}");
    }
}

/// Runs integrity check and emits progress/final frontend events.
pub async fn run_integrity_check_job(
    app: AppHandle,
    job_id: String,
    db_path: PathBuf,
    db_batch_size: usize,
    cancel_token: CancelToken,
) {
    let result = run_integrity_check(
        app.clone(),
        job_id.clone(),
        db_path,
        db_batch_size,
        &cancel_token,
    )
    .await;

    let payload = match result {
        Ok(report) => {
            let status = if report.cancelled {
                IntegrityCheckEventStatusDto::Cancelled
            } else {
                IntegrityCheckEventStatusDto::Completed
            };

            IntegrityCheckFinishedEventDto {
                job_id: job_id.clone(),
                status,
                report: Some(IntegrityCheckReportDto::from(report)),
                error: None,
            }
        }
        Err(error) => IntegrityCheckFinishedEventDto {
            job_id: job_id.clone(),
            status: IntegrityCheckEventStatusDto::Failed,
            report: None,
            error: Some(CommandError::from(error)),
        },
    };

    if let Err(error) = app.emit("integrity-check-finished", payload) {
        eprintln!("failed to emit integrity check event: {error}");
    }

    let state = app.state::<AppState>();

    if let Err(error) = state.remove_job(&job_id) {
        eprintln!("failed to remove completed integrity check job: {error}");
    }
}

/// Opens the DB and runs core manifest generation.
async fn run_manifest_generation(
    db_path: PathBuf,
    db_batch_size: usize,
    cancel_token: &CancelToken,
) -> fsdoctor_core::Result<fsdoctor_core::ManifestGenerationReport> {
    let db = ProjectDb::open(OpenProjectRequest { db_path }).await?;

    generate_manifest(
        &db,
        ManifestGenerationOptions { db_batch_size },
        cancel_token,
    )
    .await
}

/// Opens the DB and runs core integrity check.
async fn run_integrity_check(
    app: AppHandle,
    job_id: String,
    db_path: PathBuf,
    db_batch_size: usize,
    cancel_token: &CancelToken,
) -> fsdoctor_core::Result<fsdoctor_core::IntegrityCheckReport> {
    let db = ProjectDb::open(OpenProjectRequest { db_path }).await?;

    run_integrity_check_with_progress(
        &db,
        IntegrityCheckOptions { db_batch_size },
        cancel_token,
        move |progress| {
            let payload = IntegrityCheckProgressEventDto {
                job_id: job_id.clone(),
                progress: IntegrityCheckProgressDto::from(progress),
            };

            if let Err(error) = app.emit("integrity-check-progress", payload) {
                eprintln!("failed to emit integrity check progress event: {error}");
            }
        },
    )
    .await
}
