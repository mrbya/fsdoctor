use crate::{
    generate_manifest, generate_manifest_with_progress, manifest::model::ManifestGenerationOptions,
    CancelToken, CreateProjectRequest, ManifestGenerationPhase, ProjectDb,
};
use std::sync::{Arc, Mutex};

#[tokio::test]
async fn generate_manifest_persists_regular_file_with_hash(
) -> Result<(), Box<dyn std::error::Error>> {
    let temp = tempfile::tempdir()?;
    let root = temp.path().join("backup");

    std::fs::create_dir_all(&root)?;
    std::fs::write(root.join("hello.txt"), b"hello fsdoctor")?;

    let db_path = temp.path().join("project.fsdoctor.sqlite");

    let db = ProjectDb::create(CreateProjectRequest {
        db_path,
        name: "Test".to_owned(),
        root_path: root,
    })
    .await?;

    let report = generate_manifest(
        &db,
        ManifestGenerationOptions::default(),
        &CancelToken::default(),
    )
    .await?;

    let entries = db.manifest_entries_for_scan(report.scan_id).await?;

    assert!(!report.cancelled);
    assert_eq!(report.counters.total_files, 1);
    assert_eq!(report.counters.hashed_files, 1);
    assert_eq!(entries.len(), 1);
    assert_eq!(
        entries
            .first()
            .expect("should contain an entry")
            .relative_path,
        "hello.txt"
    );
    assert_eq!(
        entries.first().expect("should contain an entry").entry_kind,
        "file"
    );
    assert_eq!(
        entries.first().expect("should contain an entry").status,
        "hashed"
    );
    assert_eq!(
        entries
            .first()
            .expect("should contain an entry")
            .hash_algo
            .as_deref(),
        Some("blake3")
    );
    assert_eq!(
        entries
            .first()
            .expect("should contain an entry")
            .hash
            .as_ref()
            .map(Vec::len),
        Some(32)
    );

    Ok(())
}

#[tokio::test]
async fn generate_manifest_persists_directory_without_hash(
) -> Result<(), Box<dyn std::error::Error>> {
    let temp = tempfile::tempdir()?;
    let root = temp.path().join("backup");
    let nested = root.join("nested");

    std::fs::create_dir_all(&nested)?;

    let db = ProjectDb::create(CreateProjectRequest {
        db_path: temp.path().join("project.fsdoctor.sqlite"),
        name: "Test".to_owned(),
        root_path: root,
    })
    .await?;

    let report = generate_manifest(
        &db,
        ManifestGenerationOptions::default(),
        &CancelToken::default(),
    )
    .await?;

    let entries = db.manifest_entries_for_scan(report.scan_id).await?;

    assert_eq!(report.counters.total_dirs, 1);
    assert_eq!(entries.len(), 1);
    assert_eq!(
        entries
            .first()
            .expect("should contain an entry")
            .relative_path,
        "nested"
    );
    assert_eq!(
        entries.first().expect("should contain an entry").entry_kind,
        "directory"
    );
    assert_eq!(
        entries.first().expect("should contain an entry").status,
        "recorded"
    );
    assert_eq!(entries.first().expect("should contain an entry").hash, None);

    Ok(())
}

#[tokio::test]
async fn generate_manifest_can_be_cancelled_before_start() -> Result<(), Box<dyn std::error::Error>>
{
    let temp = tempfile::tempdir()?;
    let root = temp.path().join("backup");

    std::fs::create_dir_all(&root)?;
    std::fs::write(root.join("hello.txt"), b"hello")?;

    let db = ProjectDb::create(CreateProjectRequest {
        db_path: temp.path().join("project.fsdoctor.sqlite"),
        name: "Test".to_owned(),
        root_path: root,
    })
    .await?;

    let token = CancelToken::default();
    token.cancel();

    let report = generate_manifest(&db, ManifestGenerationOptions::default(), &token).await?;

    assert!(report.cancelled);

    Ok(())
}

#[tokio::test]
async fn generate_manifest_reports_live_progress() -> Result<(), Box<dyn std::error::Error>> {
    let temp = tempfile::tempdir()?;
    let root = temp.path().join("backup");
    let nested = root.join("nested");

    std::fs::create_dir_all(&nested)?;
    std::fs::write(root.join("hello.txt"), b"hello fsdoctor")?;

    let db = ProjectDb::create(CreateProjectRequest {
        db_path: temp.path().join("project.fsdoctor.sqlite"),
        name: "Test".to_owned(),
        root_path: root,
    })
    .await?;

    let snapshots = Arc::new(Mutex::new(Vec::new()));
    let snapshot_sink = Arc::clone(&snapshots);

    let report = generate_manifest_with_progress(
        &db,
        ManifestGenerationOptions::default(),
        &CancelToken::default(),
        move |progress| {
            snapshot_sink
                .lock()
                .expect("manifest test sink should lock")
                .push(progress);
        },
    )
    .await?;

    let snapshots = snapshots.lock().expect("manifest snapshots should lock");

    assert!(!snapshots.is_empty());
    assert!(snapshots.iter().any(|progress| {
        progress.phase == ManifestGenerationPhase::WalkingAndHashing
            && progress.files_seen >= 1
            && progress.current_path.is_some()
    }));
    assert!(snapshots.iter().any(|progress| {
        progress.phase == ManifestGenerationPhase::Writing && progress.results_written >= 1
    }));
    assert_eq!(
        snapshots.last().map(|progress| progress.phase),
        Some(ManifestGenerationPhase::Finishing)
    );
    drop(snapshots);

    assert_eq!(report.counters.total_files, 1);

    Ok(())
}

#[tokio::test]
async fn generating_manifest_twice_updates_existing_paths_without_duplicates(
) -> Result<(), Box<dyn std::error::Error>> {
    let temp = tempfile::tempdir()?;
    let root = temp.path().join("backup");

    std::fs::create_dir_all(&root)?;
    std::fs::write(root.join("hello.txt"), b"hello")?;

    let db = ProjectDb::create(CreateProjectRequest {
        db_path: temp.path().join("project.fsdoctor.sqlite"),
        name: "Test".to_owned(),
        root_path: root.clone(),
    })
    .await?;

    generate_manifest(
        &db,
        ManifestGenerationOptions::default(),
        &CancelToken::default(),
    )
    .await?;

    std::fs::write(root.join("hello.txt"), b"hello again")?;

    let second = generate_manifest(
        &db,
        ManifestGenerationOptions::default(),
        &CancelToken::default(),
    )
    .await?;

    let entries = db.manifest_entries_for_scan(second.scan_id).await?;
    let total_rows = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM manifest_entries")
        .fetch_one(&db.pool)
        .await?;
    let first_entry = entries.first().ok_or("scan should contain hello.txt")?;

    assert_eq!(entries.len(), 1);
    assert_eq!(total_rows, 1);
    assert_eq!(first_entry.relative_path, "hello.txt");

    Ok(())
}
