//! `FSDoctor` core
//!
//! Responsibilities:
//! - filesystem walking
//! - BLAKE3 hashing
//! - `SQLite` project database
//! - manifest generation
//! - integrity checking
//! - report generation
//! - CSV export
//! - future parity abstraction

#![allow(clippy::module_name_repetitions)]
// clippy WARN level lints
#![warn(
    missing_docs,
    clippy::pedantic,
    clippy::nursery,
    clippy::dbg_macro,
    clippy::unwrap_used,
    clippy::integer_division,
    clippy::large_include_file,
    clippy::map_err_ignore,
    clippy::missing_docs_in_private_items,
    clippy::panic,
    clippy::todo,
    clippy::undocumented_unsafe_blocks,
    clippy::unimplemented
)]
// clippy WARN level lints, that can be upgraded to DENY if preferred
#![warn(
    clippy::float_arithmetic,
    clippy::arithmetic_side_effects,
    clippy::modulo_arithmetic,
    clippy::as_conversions,
    clippy::assertions_on_result_states,
    clippy::clone_on_ref_ptr,
    clippy::create_dir,
    clippy::default_union_representation,
    clippy::deref_by_slicing,
    clippy::empty_drop,
    clippy::empty_structs_with_brackets,
    clippy::float_cmp_const,
    clippy::if_then_some_else_none,
    clippy::indexing_slicing,
    clippy::lossy_float_literal,
    clippy::pattern_type_mismatch,
    clippy::string_slice,
    clippy::try_err
)]
// clippy DENY level lints, they always have a quick fix that should be preferred
#![deny(
    clippy::wildcard_imports,
    clippy::rc_buffer,
    clippy::rc_mutex,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::same_name_method,
    clippy::self_named_module_files,
    clippy::shadow_unrelated,
    clippy::str_to_string,
    clippy::string_add,
    clippy::implicit_clone,
    clippy::unnecessary_self_imports,
    clippy::unneeded_field_pattern,
    clippy::unseparated_literal_suffix,
    clippy::verbose_file_reads
)]

/// `FSDoctor` backup integrity check engine.
pub(crate) mod check;
/// `FSDoctor` `SQLite` db client.
pub(crate) mod db;
/// `FSDoctor` error and result types.
pub(crate) mod error;
/// Filesystem scanning and metadata collection.
pub(crate) mod fs;
/// Streamed BLAKE3 hashing.
pub(crate) mod hash;
/// `FSDoctor` manifest management.
pub(crate) mod manifest;
/// `FSDoctor` domain model.
pub(crate) mod model;
/// `FSDoctor` path handling.
pub(crate) mod path;

// Re-exports.
// Check
pub use check::engine::{run_integrity_check, run_integrity_check_with_progress};
pub use check::model::{
    IntegrityCheckOptions, IntegrityCheckReport, IntegrityCheckSummary, DEFAULT_CHECK_DB_BATCH_SIZE,
};
pub use check::progress::{IntegrityCheckPhase, IntegrityCheckProgress};
// Db
pub use db::check::{CheckResultRecord, ExpectedManifestEntry};
pub use db::manifest::ManifestEntrySnapshot;
pub use db::project::ProjectDb;
// Error
pub use error::{Error, Result};
// Fs
pub use fs::entry::{FsEntry, FsEntryKind, FsEntryStatus, FsMetadata, SkipReason};
pub use fs::metadata::collect_metadata;
pub use fs::platform::is_reparse_point;
pub use fs::scanner::{scan_tree, ScanFlow, ScanOptions, ScanSummary};
// Hash
pub use hash::cancel::CancelToken;
pub use hash::digest::{FileDigest, HashAlgorithm};
pub use hash::file::{hash_file, FileFingerprint, HashOptions, HashOutcome, HashedFile};
// Model
pub use model::{
    CheckResultKind, CreateProjectRequest, ManifestEntryStatus, OpenProjectRequest, Project,
    ProjectId, ScanId, ScanKind, ScanStatus, CURRENT_PROJECT_FORMAT_VERSION,
};
// Manifestt
pub use manifest::generation::generate_manifest;
pub use manifest::model::{
    ManifestGenerationOptions, ManifestGenerationReport, DEFAULT_DB_BATCH_SIZE,
};
pub use manifest::progress::{ManifestGenerationPhase, ManifestGenerationProgress};
// Path
pub use path::{db_text_to_path, path_to_db_text, RelativePath};

// Tests
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests;
