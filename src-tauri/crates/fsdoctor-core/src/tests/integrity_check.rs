use std::{fs, path::PathBuf};

use sqlx::Row;
use tempfile::TempDir;

use crate::{
    generate_manifest, run_integrity_check, CancelToken, CheckResultKind, CreateProjectRequest,
    Error, IntegrityCheckOptions, ManifestGenerationOptions, ProjectDb, ScanId,
};

struct IntegrityCheckFixture {
    _temp: TempDir,
    root: PathBuf,
    db: ProjectDb,
    manifest_scan_id: ScanId,
}

impl IntegrityCheckFixture {
    async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let temp = tempfile::tempdir()?;
        let root = temp.path().join("backup");
        fs::create_dir_all(&root)?;
        fs::write(root.join("file.txt"), b"hello")?;

        let db = ProjectDb::create(CreateProjectRequest {
            db_path: temp.path().join("project.fsdoctor.sqlite"),
            name: "Test".to_owned(),
            root_path: root.clone(),
        })
        .await?;

        let manifest_report = generate_manifest(
            &db,
            ManifestGenerationOptions::default(),
            &CancelToken::default(),
        )
        .await?;

        Ok(Self {
            _temp: temp,
            root,
            db,
            manifest_scan_id: manifest_report.scan_id,
        })
    }

    fn file_path(&self) -> PathBuf {
        self.root.join("file.txt")
    }

    async fn run_check(&self) -> crate::Result<crate::IntegrityCheckReport> {
        run_integrity_check(
            &self.db,
            IntegrityCheckOptions::default(),
            &CancelToken::default(),
        )
        .await
    }
}

async fn load_check_result_kinds(
    db: &ProjectDb,
    scan_id: ScanId,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    Ok(sqlx::query_scalar::<_, String>(
        "SELECT result_kind FROM check_results WHERE scan_id = ?1 ORDER BY relative_path",
    )
    .bind(scan_id.raw())
    .fetch_all(&db.pool)
    .await?)
}

async fn count_check_results(
    db: &ProjectDb,
    scan_id: ScanId,
) -> Result<i64, Box<dyn std::error::Error>> {
    Ok(
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM check_results WHERE scan_id = ?1")
            .bind(scan_id.raw())
            .fetch_one(&db.pool)
            .await?,
    )
}

#[tokio::test]
async fn unchanged_file_is_ok() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = IntegrityCheckFixture::new().await?;

    let report = fixture.run_check().await?;

    assert_eq!(report.manifest_scan_id, fixture.manifest_scan_id);
    assert_eq!(report.summary.ok, 1);
    assert_eq!(report.summary.missing, 0);
    assert_eq!(report.summary.new, 0);
    assert_eq!(
        load_check_result_kinds(&fixture.db, report.scan_id).await?,
        vec!["ok".to_owned()]
    );

    Ok(())
}

#[tokio::test]
async fn deleted_file_is_missing() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = IntegrityCheckFixture::new().await?;
    fs::remove_file(fixture.file_path())?;

    let report = fixture.run_check().await?;

    assert_eq!(report.summary.ok, 0);
    assert_eq!(report.summary.missing, 1);
    assert_eq!(
        load_check_result_kinds(&fixture.db, report.scan_id).await?,
        vec!["missing".to_owned()]
    );

    Ok(())
}

#[tokio::test]
async fn added_file_is_new() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = IntegrityCheckFixture::new().await?;
    fs::write(fixture.root.join("new.txt"), b"new")?;

    let report = fixture.run_check().await?;

    assert_eq!(report.summary.ok, 1);
    assert_eq!(report.summary.new, 1);
    assert_eq!(
        load_check_result_kinds(&fixture.db, report.scan_id).await?,
        vec!["ok".to_owned(), "new".to_owned()]
    );

    Ok(())
}

#[tokio::test]
async fn same_size_modified_file_is_hash_mismatch() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = IntegrityCheckFixture::new().await?;
    fs::write(fixture.file_path(), b"HELLO")?;

    let report = fixture.run_check().await?;

    assert_eq!(report.summary.ok, 0);
    assert_eq!(report.summary.hash_mismatch, 1);
    assert_eq!(
        load_check_result_kinds(&fixture.db, report.scan_id).await?,
        vec!["hash_mismatch".to_owned()]
    );

    Ok(())
}

#[tokio::test]
async fn different_size_modified_file_is_size_mismatch() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = IntegrityCheckFixture::new().await?;
    fs::write(fixture.file_path(), b"hello but longer")?;

    let report = fixture.run_check().await?;

    assert_eq!(report.summary.ok, 0);
    assert_eq!(report.summary.size_mismatch, 1);
    assert_eq!(
        load_check_result_kinds(&fixture.db, report.scan_id).await?,
        vec!["size_mismatch".to_owned()]
    );

    Ok(())
}

#[tokio::test]
async fn file_replaced_by_directory_is_type_changed() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = IntegrityCheckFixture::new().await?;
    fs::remove_file(fixture.file_path())?;
    fs::create_dir_all(fixture.file_path())?;

    let report = fixture.run_check().await?;

    assert_eq!(report.summary.ok, 0);
    assert_eq!(report.summary.type_changed, 1);
    assert_eq!(
        load_check_result_kinds(&fixture.db, report.scan_id).await?,
        vec!["type_changed".to_owned()]
    );

    Ok(())
}

#[tokio::test]
async fn check_results_are_persisted() -> Result<(), Box<dyn std::error::Error>> {
    let fixture = IntegrityCheckFixture::new().await?;
    fs::write(fixture.root.join("new.txt"), b"new")?;

    let report = fixture.run_check().await?;
    let rows = sqlx::query(
        "SELECT relative_path, result_kind FROM check_results WHERE scan_id = ?1 ORDER BY relative_path",
    )
    .bind(report.scan_id.raw())
    .fetch_all(&fixture.db.pool)
    .await?;
    let mut rows = rows.into_iter();
    let first = rows
        .next()
        .expect("should persist the existing file result");
    let second = rows.next().expect("should persist the new file result");

    assert_eq!(count_check_results(&fixture.db, report.scan_id).await?, 2);
    assert!(rows.next().is_none());
    assert_eq!(first.try_get::<String, _>("relative_path")?, "file.txt");
    assert_eq!(
        first.try_get::<String, _>("result_kind")?,
        CheckResultKind::Ok.as_db_str()
    );
    assert_eq!(second.try_get::<String, _>("relative_path")?, "new.txt");
    assert_eq!(
        second.try_get::<String, _>("result_kind")?,
        CheckResultKind::New.as_db_str()
    );

    Ok(())
}

#[tokio::test]
async fn missing_completed_manifest_returns_error() -> Result<(), Box<dyn std::error::Error>> {
    let temp = tempfile::tempdir()?;
    let root = temp.path().join("backup");
    fs::create_dir_all(&root)?;
    fs::write(root.join("file.txt"), b"hello")?;

    let db = ProjectDb::create(CreateProjectRequest {
        db_path: temp.path().join("project.fsdoctor.sqlite"),
        name: "Test".to_owned(),
        root_path: root,
    })
    .await?;

    let error = run_integrity_check(
        &db,
        IntegrityCheckOptions::default(),
        &CancelToken::default(),
    )
    .await
    .expect_err("running a check without a completed manifest should fail");

    assert!(matches!(error, Error::NoCompletedManifest));

    Ok(())
}
