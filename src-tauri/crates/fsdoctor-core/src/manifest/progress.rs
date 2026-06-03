/// Manifest generation phase.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManifestGenerationPhase {
    /// Walking filesystem and hashing files.
    WalkingAndHashing,

    /// Writing manifest entries.
    Writing,

    /// Finalizing scan status.
    Finishing,
}

/// Progress snapshot for manifest generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ManifestGenerationProgress {
    /// Current phase.
    pub phase: ManifestGenerationPhase,

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
}
