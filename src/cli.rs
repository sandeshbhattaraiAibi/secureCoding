// src/cli.rs

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// A secure file backup utility in Rust
#[derive(Parser)]
#[command(name = "safe_backup", version = "0.1.0", author = "Your Name")]
pub struct Cli {
    /// Which operation to perform
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {

    Backup {

        #[arg(value_name = "SRC", value_parser = clap::value_parser!(PathBuf))]
        src: PathBuf,

        /// Path of the backup file to create
        #[arg(value_name = "DEST", value_parser = clap::value_parser!(PathBuf))]
        dest: PathBuf,
    },

    /// Restore a file from its .bak file
    Restore {

        #[arg(value_name = "BACKUP", value_parser = clap::value_parser!(PathBuf))]
        backup: PathBuf,

        /// Directory to restore into
        #[arg(value_name = "TARGET", value_parser = clap::value_parser!(PathBuf))]
        target: PathBuf,
    },

    /// Delete a regular file
    Delete {

        #[arg(value_name = "FILE", value_parser = clap::value_parser!(PathBuf))]
        file: PathBuf,
    },
}
