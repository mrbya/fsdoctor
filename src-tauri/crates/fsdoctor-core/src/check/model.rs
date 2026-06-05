use crate::ScanId;

/// Default check result DB batch size.
pub const DEFAULT_CHECK_DB_BATCH_SIZE: usize = 512;

/// Integrity-check options.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntegrityCheckOptions {
    /// Number of check results to batch per DB transaction.
    pub db_batch_size: usize,
}

impl Default for IntegrityCheckOptions {
    fn default() -> Self {
        Self {
            db_batch_size: DEFAULT_CHECK_DB_BATCH_SIZE,
        }
    }
}

/// Integrity-check result summary.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct IntegrityCheckSummary {
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

/// Final integrity-check report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegrityCheckReport {
    /// Integrity-check scan id.
    pub scan_id: ScanId,

    /// Manifest-generation scan id used as the baseline.
    pub manifest_scan_id: ScanId,

    /// Result summary.
    pub summary: IntegrityCheckSummary,

    /// Whether the check was cancelled.
    pub cancelled: bool,
}
