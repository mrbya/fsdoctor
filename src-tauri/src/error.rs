use serde::Serialize;

/// Result type used by Tauri commands.
pub type CommandResult<T> = std::result::Result<T, CommandError>;

/// Errors returned by tauri commands.
#[derive(Debug, Serialize)]
pub struct CommandError {
    /// Stable machine-readable error kind.
    pub kind: String,

    /// User-facing error message.
    pub message: String,

    /// Optional technical details.
    pub details: Option<String>,
}

impl From<fsdoctor_core::Error> for CommandError {
    fn from(error: fsdoctor_core::Error) -> Self {
        match error {
            fsdoctor_core::Error::InvalidProjectDatabase => Self {
                kind: "invalid_project_database".to_owned(),
                message: "The selected file is not a valid FSDoctor project database.".to_owned(),
                details: None,
            },
            fsdoctor_core::Error::UnsupportedFormatVersion { expected, actual } => Self {
                kind: "usupported_format_version".to_owned(),
                message: "Selected FSDoctor database has an unsupported format.".to_owned(),
                details: Some(format!(
                    "Expected format version {expected}, found {actual}."
                )),
            },
            other => Self {
                kind: "internal_error".to_owned(),
                message: "FSDoctor could not complete requested operation".to_owned(),
                details: Some(other.to_string()),
            },
        }
    }
}
