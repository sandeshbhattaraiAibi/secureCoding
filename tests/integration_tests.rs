// tests/integration_tests.rs

use assert_cmd::prelude::*;      
use predicates::prelude::*;      
use tempfile::tempdir;        
use std::fs::{self, File};
use std::io::Write;

use std::process::Command;

#[test]
fn valid_backup_restore_delete_cycle() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Setup a temp directory and a sample file
    let dir = tempdir()?;
    let src = dir.path().join("sample.txt");
    let mut file = File::create(&src)?;
    writeln!(file, "hello safe backup")?;

    // 2. Define backup path and run backup
    let bak = dir.path().join("sample.txt.bak");
    let src_str = src.to_str().unwrap();
    let bak_str = bak.to_str().unwrap();

    let mut cmd = Command::cargo_bin("safe_backup")?;
    cmd.args(&["backup", src_str, bak_str]);
    cmd.assert().success();

    // 3. Restore into a new subdirectory
    let restore_dir = dir.path().join("restored");
    let restore_dir_str = restore_dir.to_str().unwrap();

    let mut cmd = Command::cargo_bin("safe_backup")?;
    cmd.args(&["restore", bak_str, restore_dir_str]);
    cmd.assert().success();

    // Check file was restored
    let restored = restore_dir.join("sample.txt");
    assert!(restored.exists());
    let content = fs::read_to_string(&restored)?;
    assert!(content.contains("hello safe backup"));

    // 4. Delete the restored file
    let restored_str = restored.to_str().unwrap();
    let mut cmd = Command::cargo_bin("safe_backup")?;
    cmd.args(&["delete", restored_str]);
    cmd.assert().success();

    assert!(!restored.exists());
    Ok(())
}

#[test]
fn malicious_path_traversal_blocked() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Setup
    let dir = tempdir()?;
    let src = dir.path().join("good.txt");
    File::create(&src)?.write_all(b"data")?;
    let src_str = src.to_str().unwrap();

    // 2. Attempt backup with path traversal in destination
    let malicious = dir.path().join("../evil.txt.bak");
    let malicious_str = malicious.to_str().unwrap();

    let mut cmd = Command::cargo_bin("safe_backup")?;
    cmd.args(&["backup", src_str, malicious_str]);

    // 3. Should fail and emit an error about sanitization or invalid extension
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains(".bak")
            .or(predicate::str::contains("invalid")));

    Ok(())
}

#[test]
fn symlink_and_directory_deletion_prevented() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Setup directory and symlink
    let dir = tempdir()?;
    let target_file = dir.path().join("file.txt");
    File::create(&target_file)?;
    let symlink = dir.path().join("link.txt");
    #[cfg(unix)]
    std::os::unix::fs::symlink(&target_file, &symlink)?;
    #[cfg(windows)]
    std::os::windows::fs::symlink_file(&target_file, &symlink)?;

    // 2. Attempt to delete the symlink
    let symlink_str = symlink.to_str().unwrap();
    let mut cmd = Command::cargo_bin("safe_backup")?;
    cmd.args(&["delete", symlink_str]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Refusing to delete symlink"));

    // 3. Attempt to delete a directory
    let subdir = dir.path().join("subdir");
    fs::create_dir(&subdir)?;
    let subdir_str = subdir.to_str().unwrap();

    let mut cmd = Command::cargo_bin("safe_backup")?;
    cmd.args(&["delete", subdir_str]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("not a regular file"));

    Ok(())
}
