use serde::Serialize;

/// Result type used by Tauri commands.
pub type CommandResult<T> = std::result::Result<T, CommandError>;

/// Serialized command error types.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandErrorKind {
    /// Invalid `FSDoctor` project database.
    InvalidProjectDatabase,

    /// Unsupported `FSDoctor` project db format version.
    UnsupportedFormatVersion,

    /// Internal `FSDoctor` core error.
    InternalError,

    /// File/command not found.
    NotFound,
}

/// Errors returned by tauri commands.
#[derive(Debug, Clone, Serialize)]
pub struct CommandError {
    /// Stable machine-readable error kind.
    pub kind: CommandErrorKind,

    /// User-facing error message.
    pub message: String,

    /// Optional technical details.
    pub details: Option<String>,
}

impl CommandError {
    /// Creates an internal command error.
    #[must_use]
    pub fn internal(message: &str, details: Option<String>) -> Self {
        Self {
            kind: CommandErrorKind::InternalError,
            message: message.to_owned(),
            details,
        }
    }

    /// Creates a not-found command error.
    #[must_use]
    pub fn not_found(message: &str) -> Self {
        Self {
            kind: CommandErrorKind::NotFound,
            message: message.to_owned(),
            details: None,
        }
    }
}

impl From<fsdoctor_core::Error> for CommandError {
    fn from(error: fsdoctor_core::Error) -> Self {
        match error {
            fsdoctor_core::Error::InvalidProjectDatabase => Self {
                kind: CommandErrorKind::InvalidProjectDatabase,
                message: "The selected file is not a valid FSDoctor project database.".to_owned(),
                details: None,
            },
            fsdoctor_core::Error::UnsupportedFormatVersion { expected, actual } => Self {
                kind: CommandErrorKind::UnsupportedFormatVersion,
                message: "Selected FSDoctor database has an unsupported format.".to_owned(),
                details: Some(format!(
                    "Expected format version {expected}, found {actual}."
                )),
            },
            other => Self {
                kind: CommandErrorKind::InternalError,
                message: "FSDoctor could not complete requested operation".to_owned(),
                details: Some(other.to_string()),
            },
        }
    }
}
