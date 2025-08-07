// src/utils.rs

use std::{env, error::Error, path::PathBuf};
use path_clean::PathClean;


pub fn sanitize_path(input: &PathBuf) -> Result<PathBuf, Box<dyn Error>> {
    // 1. Normalize “.” and “..” segments
    let cleaned = input.clean();

    // 2. If it’s still a relative path, make it absolute against the current working dir
    let absolute = if cleaned.is_relative() {
        let cwd = env::current_dir()?;
        cwd.join(cleaned).clean()
    } else {
        cleaned
    };

    Ok(absolute)
}
