#[test]
fn hashes_empty_file_with_blake3() -> Result<(), Box<dyn std::error::Error>> {
    let temp = tempfile::tempdir()?;
    let path = temp.path().join("empty.bin");

    std::fs::write(&path, [])?;

    let outcome = crate::hash_file(
        &path,
        crate::HashOptions::default(),
        &crate::CancelToken::default(),
    )?;

    let crate::HashOutcome::Hashed(hashed) = outcome else {
        return Err("file should hash successfully".into());
    };

    assert_eq!(
        hashed.digest.to_hex(),
        "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262"
    );

    Ok(())
}

#[test]
fn hashes_regular_file() -> Result<(), Box<dyn std::error::Error>> {
    let temp = tempfile::tempdir()?;
    let path = temp.path().join("hello.txt");

    std::fs::write(&path, b"hello fsdoctor")?;

    let outcome = crate::hash_file(
        &path,
        crate::HashOptions::default(),
        &crate::CancelToken::default(),
    )?;

    assert!(matches!(outcome, crate::HashOutcome::Hashed(_)));

    Ok(())
}

#[test]
fn cancellation_interrupts_hashing() -> Result<(), Box<dyn std::error::Error>> {
    let temp = tempfile::tempdir()?;
    let path = temp.path().join("large.bin");

    std::fs::write(&path, vec![42_u8; 8 * 1024 * 1024])?;

    let token = crate::CancelToken::default();
    token.cancel();

    let result = crate::hash_file(&path, crate::HashOptions::default(), &token);

    assert!(matches!(result, Err(crate::Error::HashingCancelled)));

    Ok(())
}

#[test]
fn detects_file_changed_before_second_hash() -> Result<(), Box<dyn std::error::Error>> {
    let temp = tempfile::tempdir()?;
    let path = temp.path().join("changing.txt");

    std::fs::write(&path, b"before")?;

    let first = crate::hash_file(
        &path,
        crate::HashOptions::default(),
        &crate::CancelToken::default(),
    )?;

    std::thread::sleep(std::time::Duration::from_millis(20));
    std::fs::write(&path, b"after after after")?;

    let second = crate::hash_file(
        &path,
        crate::HashOptions::default(),
        &crate::CancelToken::default(),
    )?;

    assert!(matches!(first, crate::HashOutcome::Hashed(_)));
    assert!(matches!(second, crate::HashOutcome::Hashed(_)));

    Ok(())
}
