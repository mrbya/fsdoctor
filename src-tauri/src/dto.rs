use serde::{Deserialize, Serialize};

/// Request to create a new `FSDoctor` project.
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

/// Request to open an existing `FSDoctor` project.
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
