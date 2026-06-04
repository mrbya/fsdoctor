use std::path::PathBuf;

use fsdoctor_core::{CreateProjectRequest, OpenProjectRequest, ProjectDb, DEFAULT_DB_BATCH_SIZE};
use tauri::{AppHandle, State};

use crate::{
    dto::{
        CancelJobRequestDto, CancelJobResultDto, CreateProjectRequestDto, JobStartedDto,
        OpenProjectRequestDto, ProjectDto, StartManifestGenerationRequestDto,
    },
    error::{CommandError, CommandResult},
    handlers::run_manifest_generation_job,
    state::AppState,
};

/// Creates a new `FSDoctor` project database.
///
/// # Errors
/// Returns [`CommandError`] if the underlying db operations fail.
#[tauri::command]
pub async fn create_project(request: CreateProjectRequestDto) -> CommandResult<ProjectDto> {
    let db = ProjectDb::create(CreateProjectRequest {
        db_path: PathBuf::from(request.db_path),
        name: request.name,
        root_path: PathBuf::from(request.root_path),
    })
    .await
    .map_err(CommandError::from)?;

    db.project()
        .await
        .map(ProjectDto::from)
        .map_err(CommandError::from)
}

/// Opeens an existing `FSDoctor` project database.
///
/// # Errors
/// Returns [`CommandError`] if the underlying db operations fail.
#[tauri::command]
pub async fn open_project(request: OpenProjectRequestDto) -> CommandResult<ProjectDto> {
    let db = ProjectDb::open(OpenProjectRequest {
        db_path: PathBuf::from(request.db_path),
    })
    .await
    .map_err(CommandError::from)?;

    db.project()
        .await
        .map(ProjectDto::from)
        .map_err(CommandError::from)
}

/// Starts manifest generation as a background job.
///
/// # Errors
/// Returns [`CommandError`] if the job cannot be registered.
#[tauri::command]
pub fn start_manifest_generation(
    request: StartManifestGenerationRequestDto,
    app: AppHandle,
    state: State<'_, AppState>,
) -> CommandResult<JobStartedDto> {
    let app_state = state;
    let (job_id, cancel_token) = app_state.create_job("manifest").map_err(|details| {
        CommandError::internal("Could not create manifest job", Some(details))
    })?;

    let db_path = PathBuf::from(request.db_path);
    let db_batch_size = request.db_batch_size.unwrap_or(DEFAULT_DB_BATCH_SIZE);

    let task_job_id = job_id.clone();
    let task_app = app;

    tauri::async_runtime::spawn(async move {
        run_manifest_generation_job(task_app, task_job_id, db_path, db_batch_size, cancel_token)
            .await;
    });

    Ok(JobStartedDto { job_id })
}

/// Cancels a running job.
///
/// # Errors
/// Returns [`CommandError`] if the application job registry cannot be accessed.
#[tauri::command]
pub fn cancel_job(
    request: CancelJobRequestDto,
    state: State<'_, AppState>,
) -> CommandResult<CancelJobResultDto> {
    let app_state = state;
    let cancellation_requested = app_state
        .cancel_job(&request.job_id)
        .map_err(|details| CommandError::internal("could not cancel job.", Some(details)))?;

    Ok(CancelJobResultDto {
        job_id: request.job_id,
        cancellation_requested,
    })
}
