use time::OffsetDateTime;

use crate::{
    db::helpers::{format_timestamp, u64_to_i64},
    ProjectDb, ProjectId, Result, ScanId, ScanKind, ScanStatus,
};

/// Final counters for a scan row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ScanCounters {
    /// Total directories seen.
    pub total_dirs: u64,

    /// Total files seen.
    pub total_files: u64,

    /// Total symlinks seen.
    pub total_symlinks: u64,

    /// Total other entries seen.
    pub total_other: u64,

    /// Total bytes seen.
    pub total_bytes: u64,

    /// Number of files successfully hashed.
    pub hashed_files: u64,

    /// Number of unreadable entries.
    pub unreadable_entries: u64,

    /// Number of files that changed during scan.
    pub changed_during_scan: u64,
}

impl ProjectDb {
    /// Creates a running scan row.
    ///
    /// # Errors
    /// Returns [`Error`] if:
    /// - underlying DB operations fail,
    /// - fails to format timestamp.
    pub async fn create_scan(&self, project_id: ProjectId, kind: ScanKind) -> Result<ScanId> {
        let started_at = format_timestamp(OffsetDateTime::now_utc())?;

        let result = sqlx::query(
            r"
            INSERT INTO scans (
                project_id,
                kind,
                status,
                started_at
            )
            VALUES (?1, ?2, ?3, ?4)
            ",
        )
        .bind(project_id.raw())
        .bind(kind.as_db_str())
        .bind(ScanStatus::Running.as_db_str())
        .bind(started_at)
        .execute(&self.pool)
        .await?;

        Ok(ScanId::from_raw(result.last_insert_rowid()))
    }

    /// Completes a scan row.
    ///
    /// # Errors
    /// Returns [`Error`] if underlying DB operations fail.
    pub async fn finish_scan(
        &self,
        scan_id: ScanId,
        status: ScanStatus,
        counters: ScanCounters,
        error_message: Option<&str>,
    ) -> Result<()> {
        let finished_at = format_timestamp(OffsetDateTime::now_utc())?;

        sqlx::query(
            r"
            UPDATE scans
            SET
                status = ?1,
                finished_at = ?2,
                total_dirs = ?3,
                total_files = ?4,
                total_symlinks = ?5,
                total_other = ?6,
                total_bytes = ?7,
                hashed_files = ?8,
                unreadable_entries = ?9,
                changed_during_scan = ?10,
                error_message = ?11
            WHERE id = ?12
            ",
        )
        .bind(status.as_db_str())
        .bind(finished_at)
        .bind(u64_to_i64(counters.total_dirs)?)
        .bind(u64_to_i64(counters.total_files)?)
        .bind(u64_to_i64(counters.total_symlinks)?)
        .bind(u64_to_i64(counters.total_other)?)
        .bind(u64_to_i64(counters.total_bytes)?)
        .bind(u64_to_i64(counters.hashed_files)?)
        .bind(u64_to_i64(counters.unreadable_entries)?)
        .bind(u64_to_i64(counters.changed_during_scan)?)
        .bind(error_message)
        .bind(scan_id.raw())
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
