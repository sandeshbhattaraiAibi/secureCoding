// src/restore.rs

use std::{fs, io};
use std::path::PathBuf;
use crate::utils;

/// Perform the “restore” operation: copy from a `.bak` file back into a target directory
pub fn run(backup_path: &PathBuf, target_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Sanitize both paths
// src/restore.rs

use std::{fs, io};
use std::path::PathBuf;
use crate::utils;

/// Perform the “restore” operation: copy from a `.bak` file back into a target directory
pub fn run(backup_path: &PathBuf, target_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Sanitize both paths
    let backup = utils::sanitize_path(backup_path)?;
    let target = utils::sanitize_path(target_dir)?;

    // 2. Ensure backup exists, is not a symlink, and is a regular file
    let meta = fs::symlink_metadata(&backup)?;
    if meta.file_type().is_symlink() {
        return Err(format!("Refusing to restore symlink: {}", backup.display()).into());
    }
    if !meta.is_file() {
        return Err(format!("Backup path is not a file: {}", backup.display()).into());
    }

    // 3. Enforce `.bak` extension on the backup file
    if backup.extension().and_then(|e| e.to_str()).unwrap_or("") != "bak" {
        return Err(format!(
            "Backup file does not have a `.bak` extension: {}",
            backup.display()
        )
        .into());
    }

    // 4. Derive original filename by stripping the “.bak” suffix
    let file_name = backup
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or("Invalid backup file name")?;
    let orig_name = file_name
        .strip_suffix(".bak")
        .ok_or(format!("Filename missing `.bak` suffix: {}", file_name))?;

    // 5. Create target directory (and parents) if it doesn’t exist
    fs::create_dir_all(&target)?;

    // 6. Prevent overwriting an existing file in the target
    let dest_path = target.join(orig_name);
    if dest_path.exists() {
        return Err(format!(
            "Destination file already exists: {}",
            dest_path.display()
        )
        .into());
    }

    // 7. Stream-copy from backup file to the destination
    let mut reader = fs::File::open(&backup)?;
    let mut writer = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&dest_path)?;
    io::copy(&mut reader, &mut writer)?;

    // 8. Log the successful restore
    log::info!("Restore completed: {} → {}", backup.display(), dest_path.display());

    Ok(())


}


