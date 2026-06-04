use sqlx::Row;

use crate::{
    db::helpers::{optional_i128_to_i64, optional_u64_to_i64},
    FsEntryKind, ManifestEntryStatus, ProjectDb, ProjectId, RelativePath, Result, ScanId,
};

/// Database-ready manifest entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestEntryRecord {
    /// Project id.
    pub project_id: ProjectId,

    /// Last scan that observed this entry.
    pub scan_id: ScanId,

    /// Root-relative path.
    pub relative_path: RelativePath,

    /// Entry kind.
    pub entry_kind: FsEntryKind,

    /// Size for regular files.
    pub size_bytes: Option<u64>,

    /// Modified time in nanoseconds.
    pub mtime_ns: Option<i128>,

    /// Readonly flag.
    pub readonly: bool,

    /// Hash algorithm DB string.
    pub hash_algo: Option<&'static str>,

    /// Raw hash bytes.
    pub hash: Option<[u8; 32]>,

    /// Manifest status.
    pub status: ManifestEntryStatus,

    /// Optional error message.
    pub error_message: Option<String>,
}

/// Persisted manifest entry snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestEntrySnapshot {
    /// Relative path string stored in the database.
    pub relative_path: String,

    /// Entry kind DB string.
    pub entry_kind: String,

    /// Hash algorithm DB string.
    pub hash_algo: Option<String>,

    /// Raw hash bytes.
    pub hash: Option<Vec<u8>>,

    /// Manifest status DB string.
    pub status: String,

    /// Optional error message.
    pub error_message: Option<String>,
}

impl ProjectDb {
    /// Upserts one manifest entry.
    ///
    /// # Errors
    /// Returns [`crate::error::Error`] if:
    /// - underlying DB operations fail,
    /// - integer conversions fail.
    pub async fn upsert_manifest_entry(&self, entry: &ManifestEntryRecord) -> Result<()> {
        sqlx::query(
            r"
            INSERT INTO manifest_entries (
                project_id,
                relative_path,
                entry_kind,
                size_bytes,
                mtime_ns,
                readonly,
                hash_algo,
                hash,
                last_seen_scan_id,
                status,
                error_message
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
            ON CONFLICT(project_id, relative_path)
            DO UPDATE SET
                entry_kind = excluded.entry_kind,
                size_bytes = excluded.size_bytes,
                mtime_ns = excluded.mtime_ns,
                readonly = excluded.readonly,
                hash_algo = excluded.hash_algo,
                hash = excluded.hash,
                last_seen_scan_id = excluded.last_seen_scan_id,
                status = excluded.status,
                error_message = excluded.error_message
            ",
        )
        .bind(entry.project_id.raw())
        .bind(entry.relative_path.as_str())
        .bind(fs_entry_kind_db_str(entry.entry_kind))
        .bind(optional_u64_to_i64(entry.size_bytes)?)
        .bind(optional_i128_to_i64(entry.mtime_ns)?)
        .bind(i64::from(entry.readonly))
        .bind(entry.hash_algo)
        .bind(entry.hash.map(Vec::from))
        .bind(entry.scan_id.raw())
        .bind(entry.status.as_db_str())
        .bind(entry.error_message.as_deref())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Upserts manifest entries in a single transaction.
    ///
    /// # Errors
    /// Returns [`crate::error::Error`] if:
    /// - underlying DB operations fail,
    /// - integer conversions fail.
    pub async fn upsert_manifest_entries(&self, entries: &[ManifestEntryRecord]) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        for entry in entries {
            sqlx::query(
                r"
                INSERT INTO manifest_entries (
                    project_id,
                    relative_path,
                    entry_kind,
                    size_bytes,
                    mtime_ns,
                    readonly,
                    hash_algo,
                    hash,
                    last_seen_scan_id,
                    status,
                    error_message
                )
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
                ON CONFLICT(project_id, relative_path)
                DO UPDATE SET
                    entry_kind = excluded.entry_kind,
                    size_bytes = excluded.size_bytes,
                    mtime_ns = excluded.mtime_ns,
                    readonly = excluded.readonly,
                    hash_algo = excluded.hash_algo,
                    hash = excluded.hash,
                    last_seen_scan_id = excluded.last_seen_scan_id,
                    status = excluded.status,
                    error_message = excluded.error_message
                ",
            )
            .bind(entry.project_id.raw())
            .bind(entry.relative_path.as_str())
            .bind(fs_entry_kind_db_str(entry.entry_kind))
            .bind(optional_u64_to_i64(entry.size_bytes)?)
            .bind(optional_i128_to_i64(entry.mtime_ns)?)
            .bind(i64::from(entry.readonly))
            .bind(entry.hash_algo)
            .bind(entry.hash.map(Vec::from))
            .bind(entry.scan_id.raw())
            .bind(entry.status.as_db_str())
            .bind(entry.error_message.as_deref())
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        Ok(())
    }

    /// Returns manifest entries last seen by a scan.
    ///
    /// This is useful for reports and tests.
    ///
    /// # Errors
    /// Returns [`crate::error::Error`] if the database query fails.
    pub async fn manifest_entries_for_scan(
        &self,
        scan_id: ScanId,
    ) -> Result<Vec<ManifestEntrySnapshot>> {
        let rows = sqlx::query(
            r"
        SELECT relative_path, entry_kind, hash_algo, hash, status, error_message
        FROM manifest_entries
        WHERE last_seen_scan_id = ?1
        ORDER BY relative_path
        ",
        )
        .bind(scan_id.raw())
        .fetch_all(&self.pool)
        .await?;

        let mut entries = Vec::with_capacity(rows.len());

        for row in rows {
            entries.push(ManifestEntrySnapshot {
                relative_path: row.try_get("relative_path")?,
                entry_kind: row.try_get("entry_kind")?,
                hash_algo: row.try_get("hash_algo")?,
                hash: row.try_get("hash")?,
                status: row.try_get("status")?,
                error_message: row.try_get("error_message")?,
            });
        }

        Ok(entries)
    }
}

/// Stable DB string for FS entry kind.
const fn fs_entry_kind_db_str(kind: FsEntryKind) -> &'static str {
    match kind {
        FsEntryKind::File => "file",
        FsEntryKind::Directory => "directory",
        FsEntryKind::Symlink => "symlink",
        FsEntryKind::Other => "other",
    }
}
