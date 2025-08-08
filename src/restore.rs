// src/restore.rs

use std::{fs, io};
use std::path::PathBuf;
use crate::utils;

/// Perform the “restore” operation: copy from a `.bak` file back into a target directory
pub fn run(backup_path: &PathBuf, target_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Sanitize both paths

}

