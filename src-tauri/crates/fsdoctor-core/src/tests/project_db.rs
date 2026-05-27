use std::path::PathBuf;

use crate::{
    CreateProjectRequest, Error, OpenProjectRequest, ProjectDb, CURRENT_PROJECT_FORMAT_VERSION,
};
use pretty_assertions::assert_eq;
use tempfile::tempdir;

#[tokio::test]
async fn create_project_creates_database_file() {
    let temp = tempdir().expect("tempdir should be created");
    let db_path = temp.path().join("backup.fsdoctor.sqlite");
    let root_path = temp.path().join("backup-root");

    let request = CreateProjectRequest {
        db_path: db_path.clone(),
        name: "Test backup".to_owned(),
        root_path,
    };

    let db = ProjectDb::create(request)
        .await
        .expect("project database should be created");

    let project = db.project().await.expect("project should be readable");

    assert!(db_path.exists());
    assert_eq!(project.name, "Test backup");
    assert_eq!(project.format_version, CURRENT_PROJECT_FORMAT_VERSION);
}

#[tokio::test]
async fn open_project_reads_existing_metadata() {
    let temp = tempdir().expect("tempdir should be created");
    let db_path = temp.path().join("backup.fsdoctor.sqlite");
    let root_path = PathBuf::from("/tmp/backup-root");

    let create_request = CreateProjectRequest {
        db_path: db_path.clone(),
        name: "Archive".to_owned(),
        root_path: root_path.clone(),
    };

    ProjectDb::create(create_request)
        .await
        .expect("project database should be created");

    let db = ProjectDb::open(OpenProjectRequest { db_path })
        .await
        .expect("project database should open");

    let project = db.project().await.expect("project should be readable");

    assert_eq!(project.name, "Archive");
    assert_eq!(project.root_path, root_path);
}

#[tokio::test]
async fn open_rejects_non_fsdoctor_sqlite_database() {
    let temp = tempdir().expect("tempdir should be created");
    let db_path = temp.path().join("foreign.sqlite");

    let pool = sqlx::SqlitePool::connect_with(
        sqlx::sqlite::SqliteConnectOptions::new()
            .filename(&db_path)
            .create_if_missing(true),
    )
    .await
    .expect("foreign sqlite database should be created");

    sqlx::query("CREATE TABLE unrelated (id INTEGER PRIMARY KEY)")
        .execute(&pool)
        .await
        .expect("foreign table should be created");

    pool.close().await;

    let result = ProjectDb::open(OpenProjectRequest { db_path })
        .await
        .expect_err("non-fsdoctor db opening should error");

    assert!(matches!(result, Error::Database(_)));
}
