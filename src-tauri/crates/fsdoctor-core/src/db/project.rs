use std::path::Path;

use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};
use sqlx::Row;
use sqlx::{migrate::Migrator, SqlitePool};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

use crate::error::{Error, Result};
use crate::{
    db_text_to_path, path_to_db_text, CreateProjectRequest, OpenProjectRequest, Project, ProjectId,
    CURRENT_PROJECT_FORMAT_VERSION,
};

/// Embedded SQL migrations.
static MIGRATOR: Migrator = sqlx::migrate!("../../migrations");

/// `FSDoctor` project database handle.
#[derive(Debug, Clone)]
pub struct ProjectDb {
    /// `SQLite` connection pool.
    pool: SqlitePool,
}

impl ProjectDb {
    /// Creates a new `FSDoctor` project database.
    ///
    /// Existing databases are opened and initialized if they do not yet contain
    /// `FSDoctor` metadata. Callers should avoid using this method for arbitrary
    /// existing `SQLite` databases.
    ///
    /// # Errors
    /// Returns [`crate::error::Error`] if:
    /// - fails to connect to db,
    /// - fails to initialize project,
    /// - fails to commit project to db.
    pub async fn create(request: CreateProjectRequest) -> Result<Self> {
        let pool = connect_sqlite(&request.db_path, true).await?;

        MIGRATOR.run(&pool).await?;

        let db = Self { pool };

        db.initialize_project(&request).await?;
        db.verify_format_version().await?;

        Ok(db)
    }

    /// Opens an existing `FSDoctor` project databvase.
    ///
    /// # Errors
    /// Returns [`crate::error::Error`] if:
    /// - fails to connect to db,
    /// - fails to verify project marker,
    /// - project has invalid format version,
    /// - db contains multiple projects.
    pub async fn open(request: OpenProjectRequest) -> Result<Self> {
        let pool = connect_sqlite(&request.db_path, false).await?;
        let db = Self { pool };

        db.verify_project_marker().await?;
        db.verify_format_version().await?;
        db.verify_single_project().await?;

        Ok(db)
    }

    /// Returns the project stored in this database.
    ///
    /// # Errors
    /// Returns [`Error`] if:
    /// - fails to fetch project metadata,
    /// - fails to construct db path,
    /// - fails to parse timestamps.
    pub async fn project(&self) -> Result<Project> {
        let row = sqlx::query(
            r"
            SELECT id, name, root_path, created_at, updated_at, manifest_format_version
            FROM projects
            LIMIT 1
            ",
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or(Error::InvalidProjectDatabase)?;

        let id: i64 = row.try_get("id")?;
        let name: String = row.try_get("name")?;
        let root_path: String = row.try_get("root_path")?;
        let created_at: String = row.try_get("created_at")?;
        let updated_at: String = row.try_get("updated_at")?;
        let format_version: i64 = row.try_get("manifest_format_version")?;

        Ok(Project {
            id: ProjectId::from_raw(id),
            name,
            root_path: db_text_to_path(&root_path),
            created_at: parse_timestamp(&created_at)?,
            updated_at: parse_timestamp(&updated_at)?,
            format_version,
        })
    }

    /// Initializes `FSDoctor` metadata and project row.
    async fn initialize_project(&self, request: &CreateProjectRequest) -> Result<()> {
        let now = OffsetDateTime::now_utc();
        let now_text = format_timestamp(now)?;
        let root_path = path_to_db_text(&request.root_path)?;

        let mut tx = self.pool.begin().await?;

        sqlx::query(
            r"
            INSERT OR REPLACE INTO app_meta (key, value)
            VALUES ('fsdoctor.schema', 'project')
            ",
        )
        .execute(&mut *tx)
        .await?;

        sqlx::query(
            r"
            INSERT OR REPLACE INTO app_meta (key, value)
            VALUES ('fsdoctor.format_version', ?1)
            ",
        )
        .bind(CURRENT_PROJECT_FORMAT_VERSION.to_string())
        .execute(&mut *tx)
        .await?;

        sqlx::query(
            r"
            INSERT INTO projects (
                name,
                root_path,
                created_at,
                updated_at,
                manifest_format_version
            )
            VALUES (?1, ?2, ?3, ?4, ?5)
            ",
        )
        .bind(&request.name)
        .bind(root_path)
        .bind(&now_text)
        .bind(&now_text)
        .bind(CURRENT_PROJECT_FORMAT_VERSION)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(())
    }

    /// Verifies that the database contains the `FSDoctor` project marker.
    async fn verify_project_marker(&self) -> Result<()> {
        let value = sqlx::query_scalar::<_, String>(
            r"
            SELECT value
            FROM app_meta
            WHERE key = 'fsdoctor.schema'
            ",
        )
        .fetch_optional(&self.pool)
        .await?;

        match value.as_deref() {
            Some("project") => Ok(()),
            _ => Err(Error::InvalidProjectDatabase),
        }
    }

    /// Verifies the database format version.
    async fn verify_format_version(&self) -> Result<()> {
        let value = sqlx::query_scalar::<_, String>(
            r"
            SELECT value
            FROM app_meta
            WHERE key = 'fsdoctor.format_version'
            ",
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or(Error::InvalidProjectDatabase)?;

        let actual = value
            .parse::<i64>()
            .map_err(|_error| Error::InvalidProjectDatabase)?;

        if actual != CURRENT_PROJECT_FORMAT_VERSION {
            return Err(Error::UnsupportedFormatVersion {
                expected: CURRENT_PROJECT_FORMAT_VERSION,
                actual,
            });
        }

        Ok(())
    }

    /// Verifies that the database contains exactly 1 project.
    async fn verify_single_project(&self) -> Result<()> {
        let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM projects")
            .fetch_one(&self.pool)
            .await?;

        if count != 1 {
            return Err(Error::UnsupportedProjectCount { count });
        }

        Ok(())
    }
}

/// Opens `SQLite` db connection pool.
async fn connect_sqlite(path: &Path, create_if_missing: bool) -> Result<SqlitePool> {
    let options = SqliteConnectOptions::new()
        .filename(path)
        .create_if_missing(create_if_missing)
        .journal_mode(SqliteJournalMode::Wal)
        .foreign_keys(true);

    Ok(SqlitePool::connect_with(options).await?)
}

/// Formats a timestamp for storage.
fn format_timestamp(timestamp: OffsetDateTime) -> Result<String> {
    timestamp
        .format(&Rfc3339)
        .map_err(|_error| Error::InvalidProjectDatabase)
}

/// Parses a timestamp from storage.
fn parse_timestamp(timestamp: &str) -> Result<OffsetDateTime> {
    OffsetDateTime::parse(timestamp, &Rfc3339).map_err(|_error| Error::InvalidProjectDatabase)
}
