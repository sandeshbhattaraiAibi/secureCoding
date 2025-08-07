// src/main.rs

mod cli;
mod backup;
mod restore;
mod delete;
mod logging;
mod utils;

use clap::Parser;
use cli::{Cli, Commands};
use logging::init_logging;
use std::{process, panic};

fn main() {
    // 1. Initialize logging to logfile.txt
    if let Err(e) = init_logging() {
        eprintln!("Failed to initialize logger: {}", e);
        process::exit(1);
    }

    // 2. Catch any unexpected panic and log it
    panic::set_hook(Box::new(|info| {
        log::error!("Panic occurred: {}", info);
    }));

    // 3. Parse command‐line arguments
    let cli = Cli::parse();

    // 4. Dispatch to the right subcommand and run it
    let result = match cli.command {
        Commands::Backup { src, dest } => {
            log::info!(
                "Backup requested: {} → {}",
                src.display(),
                dest.display()
            );
            backup::run(&src, &dest)
        }
        Commands::Restore { backup, target } => {
            log::info!(
                "Restore requested: {} → {}",
                backup.display(),
                target.display()
            );
            restore::run(&backup, &target)
        }
        Commands::Delete { file } => {
            log::info!("Delete requested: {}", file.display());
            delete::run(&file)
        }
    };

    // 5. Handle the outcome: log success or error, and set exit code
    match result {
        Ok(_) => {
            log::info!("Operation completed successfully");
            // exit code 0
        }
        Err(err) => {
            log::error!("Operation failed: {}", err);
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    }
}
