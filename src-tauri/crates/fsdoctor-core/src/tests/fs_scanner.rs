use std::fs;

use crate::{scan_tree, Error, FsEntryKind, ScanFlow, ScanOptions};
use pretty_assertions::assert_eq;

#[test]
fn scanner_discovers_files_and_directories() -> Result<(), Box<dyn std::error::Error>> {
    let temp = tempfile::tempdir()?;
    let root = temp.path();

    fs::create_dir_all(root.join("dir"))?;
    fs::write(root.join("dir").join("file.txt"), b"hello")?;
    fs::write(root.join("root.txt"), b"root")?;

    let mut entries = Vec::new();

    let summary = scan_tree(root, ScanOptions::default(), |entry| {
        entries.push(entry);
        Ok(ScanFlow::Continue)
    })?;

    assert_eq!(summary.files, 2);
    assert_eq!(summary.directories, 1);

    let paths = entries
        .iter()
        .map(|entry| entry.relative_path.as_str())
        .collect::<Vec<_>>();

    assert!(paths.contains(&"dir"));
    assert!(paths.contains(&"dir/file.txt"));
    assert!(paths.contains(&"root.txt"));

    Ok(())
}

#[test]
fn scanner_does_not_emit_scan_root() -> Result<(), Box<dyn std::error::Error>> {
    let temp = tempfile::tempdir()?;
    let root = temp.path();

    fs::write(root.join("file.txt"), b"hello")?;

    let mut entries = Vec::new();

    scan_tree(root, ScanOptions::default(), |entry| {
        entries.push(entry);
        Ok(ScanFlow::Continue)
    })?;

    assert_eq!(entries.len(), 1);
    assert_eq!(
        entries
            .first()
            .expect("should contain an entry")
            .relative_path
            .as_str(),
        "file.txt"
    );

    Ok(())
}

#[test]
fn scanner_rejects_file_as_root() -> Result<(), Box<dyn std::error::Error>> {
    let temp = tempfile::tempdir()?;
    let file_path = temp.path().join("not-a-directory.txt");
    fs::write(&file_path, b"hello")?;

    let result = scan_tree(&file_path, ScanOptions::default(), |_entry| {
        Ok(ScanFlow::Continue)
    })
    .expect_err("scan should fail on root being a file");

    assert!(matches!(result, Error::InvalidScanRoot { .. }));

    Ok(())
}

#[test]
fn scanner_can_be_stopped_by_callback() -> Result<(), Box<dyn std::error::Error>> {
    let temp = tempfile::tempdir()?;
    let root = temp.path();

    fs::write(root.join("one.txt"), b"one")?;
    fs::write(root.join("two.txt"), b"two")?;

    let mut count = 0_u64;

    scan_tree(root, ScanOptions::default(), |_entry| {
        count = count.saturating_add(1);
        Ok(ScanFlow::Stop)
    })?;

    assert_eq!(count, 1);

    Ok(())
}

#[cfg(unix)]
#[test]
fn scanner_skips_symlinks_by_default() -> Result<(), Box<dyn std::error::Error>> {
    use std::os::unix::fs::symlink;

    let temp = tempfile::tempdir()?;
    let root = temp.path();

    fs::write(root.join("target.txt"), b"target")?;
    symlink(root.join("target.txt"), root.join("link.txt"))?;

    let mut entries = Vec::new();

    scan_tree(root, ScanOptions::default(), |entry| {
        entries.push(entry);
        Ok(ScanFlow::Continue)
    })?;

    let link = entries
        .iter()
        .find(|entry| entry.relative_path.as_str() == "link.txt")
        .ok_or("link entry should exist")?;

    assert_eq!(link.kind, FsEntryKind::Symlink);

    Ok(())
}
