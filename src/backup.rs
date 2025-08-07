// src/backup.rs

use std::{fs, io};
use std::path::PathBuf;
use crate::utils;

/// Perform the “backup” operation: copy `src` to `dest` (must end in `.bak`)
pub fn run(src: &PathBuf, dest: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Sanitize both source and destination paths
    let src = utils::sanitize_path(src)?;
    let dest = utils::sanitize_path(dest)?;


    let meta = fs::symlink_metadata(&src)?;
    if !meta.is_file() {
        return Err(format!("Source is not a regular file: {}", src.display()).into());
    }

    // 3. Enforce `.bak` extension on destination
    if dest.extension().and_then(|e| e.to_str()).unwrap_or("") != "bak" {
        return Err(format!(
            "Destination must have a `.bak` extension: {}",
            dest.display()
        )
        .into());
    }

    // 4. Create parent directories of the destination if needed
    if let Some(parent_dir) = dest.parent() {
        fs::create_dir_all(parent_dir)?;
    }

    // 5. Prevent overwriting an existing backup
    if dest.exists() {
        return Err(format!("Backup file already exists: {}", dest.display()).into());
    }

    // 6. Open source for reading and destination for writing
    let mut reader = fs::File::open(&src)?;
    let mut writer = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&dest)?;

    // 7. Copy data in a streaming fashion
    io::copy(&mut reader, &mut writer)?;

    // 8. Log the successful operation
    log::info!("Backup completed: {} → {}", src.display(), dest.display());

    Ok(())
}
