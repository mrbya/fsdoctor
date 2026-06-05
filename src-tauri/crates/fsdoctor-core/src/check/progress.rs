use crate::IntegrityCheckSummary;

/// Integrity-check phase.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrityCheckPhase {
    /// Loading the latest completed manifest from the database.
    LoadingManifest,

    /// Walking the current filesystem tree and comparing entries.
    WalkingAndChecking,

    /// Recording manifest entries that are missing from the current tree.
    RecordingMissingEntries,

    /// Writing check results to `SQLite.
    Writing,

    /// Finalizing scan status.
    Finishing,
}

/// Integrity-check progress snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegrityCheckProgress {
    /// Current phase.
    pub phase: IntegrityCheckPhase,

    /// Current path, if available.
    pub current_path: Option<String>,

    /// Current summary counters.
    pub summary: IntegrityCheckSummary,

    /// Files seen in the current tree.
    pub files_seen: u64,

    /// Directories seen in the current tree.
    pub dirs_seen: u64,

    /// Bytes seen in the current tree.
    pub bytes_seen: u64,

    /// Files hashed during check.
    pub files_hashed: u64,

    /// Results written to the database.
    pub results_written: u64,
}
