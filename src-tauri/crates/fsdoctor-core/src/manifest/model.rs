use crate::{db::scan::ScanCounters, model::ScanId};

/// Default manifest generation DB batch size.
pub const DEFAULT_DB_BATCH_SIZE: usize = 512;

/// Options for manifest generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ManifestGenerationOptions {
    /// Number of manifest entries to batch per DB transaction.
    pub db_batch_size: usize,
}

impl Default for ManifestGenerationOptions {
    fn default() -> Self {
        Self {
            db_batch_size: DEFAULT_DB_BATCH_SIZE,
        }
    }
}

/// Final manifest generation report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestGenerationReport {
    /// Scan id.
    pub scan_id: ScanId,

    /// Final counters.
    pub counters: ScanCounters,

    /// Was the generation cancelled?
    pub cancelled: bool,
}
