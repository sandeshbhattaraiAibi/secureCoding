// src/delete.rs

use std::fs;
use std::path::PathBuf;
use crate::utils;


pub fn run(file: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Sanitize the path
  let file = utils::sanitize_path(file)?;
    
    // 2. Ensure it exists and is not a symlink
    let meta = fs::symlink_metadata(&file)?;
    if meta.file_type().is_symlink() {
        return Err(format!("Refusing to delete symlink: {}", file.display()).into());
    }
    
    // 3. Ensure it's a regular file
    if !meta.is_file() {
        return Err(format!("Path is not a regular file: {}", file.display()).into());
    }

    // 4. Delete the file
    fs::remove_file(&file)?;

    // 5. Log the successful deletion
    log::info!("Deleted file: {}", file.display());

    Ok(())
}


