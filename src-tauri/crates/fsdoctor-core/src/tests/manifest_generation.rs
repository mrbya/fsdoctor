use crate::{
    generate_manifest, manifest::model::ManifestGenerationOptions, CancelToken,
    CreateProjectRequest, ProjectDb,
};

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
