use std::collections::HashMap;

use sqlx::Row;

use crate::{
    db::helpers::optional_u64_to_i64, CheckResultKind, Error, FsEntryKind, ProjectDb, ProjectId,
    Result, ScanId,
};

/// Manifest entry loaded for integrity checking.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpectedManifestEntry {
    /// Root-relative path string.
    pub relative_path: String,

    /// Expected entry kind.
    pub entry_kind: FsEntryKind,

    /// Expected file size.
    pub size_bytes: Option<u64>,

    /// Expected raw hash bytes.
    pub hash: Option<[u8; 32]>,

    /// Expected hash algorithm.
    pub hash_algo: Option<String>,
}

/// Database-ready integrity check result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckResultRecord {
    /// Integrity-check scan id.
    pub scan_id: ScanId,

    /// Root-relative path.
    pub relative_path: String,

    /// Result kind.
    pub result_kind: CheckResultKind,

    /// Expected entry kind.
    pub expected_entry_kind: Option<FsEntryKind>,

    /// Actual entry kind.
    pub actual_entry_kind: Option<FsEntryKind>,

    /// Expected file size.
    pub expected_size_bytes: Option<u64>,

    /// Actual file size.
    pub actual_size_bytes: Option<u64>,

    /// Expected hash.
    pub expected_hash: Option<[u8; 32]>,

    /// Actual hash.
    pub actual_hash: Option<[u8; 32]>,

    /// Optional message.
    pub message: Option<String>,
}

impl ProjectDb {
    /// Returns the latest completed manifest-generation scan for a project.
    ///
    /// # Errors
    /// Returns an error if the database query fails.
    pub async fn latest_completed_manifest_scan(
        &self,
        project_id: ProjectId,
    ) -> Result<Option<ScanId>> {
        let scan_id = sqlx::query_scalar::<_, i64>(
            r"
            SELECT id
            FROM scans
            WHERE project_id = ?1
              AND kind = 'manifest_generation'
              AND status = 'completed'
            ORDER BY started_at DESC, id DESC
            LIMIT 1
            ",
        )
        .bind(project_id.raw())
        .fetch_optional(&self.pool)
        .await?;

        Ok(scan_id.map(ScanId::from_raw))
    }

    /// Loads manifest entries from a completed manifest scan.
    ///
    /// # Errors
    /// Returns an error if the database query fails or stored values are invalid.
    pub async fn load_manifest_for_check(
        &self,
        project_id: ProjectId,
        manifest_scan_id: ScanId,
    ) -> Result<HashMap<String, ExpectedManifestEntry>> {
        let rows = sqlx::query(
            r"
            SELECT relative_path, entry_kind, size_bytes, hash_algo, hash
            FROM manifest_entries
            WHERE project_id = ?1
              AND last_seen_scan_id = ?2
            ",
        )
        .bind(project_id.raw())
        .bind(manifest_scan_id.raw())
        .fetch_all(&self.pool)
        .await?;

        let mut entries = HashMap::with_capacity(rows.len());

        for row in rows {
            let relative_path: String = row.try_get("relative_path")?;
            let entry_kind_text: String = row.try_get("entry_kind")?;
            let size_bytes: Option<i64> = row.try_get("size_bytes")?;
            let hash_blob: Option<Vec<u8>> = row.try_get("hash")?;

            let hash = hash_blob.map(vec_to_digest).transpose()?;

            let entry = ExpectedManifestEntry {
                relative_path: relative_path.clone(),
                entry_kind: FsEntryKind::from_db_str(&entry_kind_text)?,
                size_bytes: size_bytes
                    .map(u64::try_from)
                    .transpose()
                    .map_err(|_error| Error::NumericOverflow)?,
                hash,
                hash_algo: row.try_get("hash_algo")?,
            };

            entries.insert(relative_path, entry);
        }

        Ok(entries)
    }

    /// Inserts check results in a single transaction.
    ///
    /// # Errors
    /// Returns an error if the database write fails.
    pub async fn insert_check_result(&self, results: &[CheckResultRecord]) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        for result in results {
            sqlx::query(
                r"
                INSERT INTO check_results (
                    scan_id,
                    relative_path,
                    result_kind,
                    expected_entry_kind,
                    actual_entry_kind,
                    expected_size_bytes,
                    actual_size_bytes,
                    expected_hash,
                    actual_hash,
                    message
                )
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
                ",
            )
            .bind(result.scan_id.raw())
            .bind(&result.relative_path)
            .bind(result.result_kind.as_db_str())
            .bind(result.expected_entry_kind.map(FsEntryKind::as_db_str))
            .bind(result.actual_entry_kind.map(FsEntryKind::as_db_str))
            .bind(optional_u64_to_i64(result.expected_size_bytes)?)
            .bind(optional_u64_to_i64(result.actual_size_bytes)?)
            .bind(result.expected_hash.map(Vec::from))
            .bind(result.actual_hash.map(Vec::from))
            .bind(result.message.as_deref())
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        Ok(())
    }
}

/// Converts a vector of bytes to a hash digest.
fn vec_to_digest(bytes: Vec<u8>) -> Result<[u8; 32]> {
    let array: [u8; 32] = bytes
        .try_into()
        .map_err(|_bytes: Vec<u8>| Error::InvalidProjectDatabase)?;

    Ok(array)
}
