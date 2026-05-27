use std::path::PathBuf;

use fsdoctor_core::{CreateProjectRequest, OpenProjectRequest, ProjectDb};

use crate::{
    dto::{CreateProjectRequestDto, OpenProjectRequestDto, ProjectDto},
    error::{CommandError, CommandResult},
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
