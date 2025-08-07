// src/logging.rs

use simplelog::{WriteLogger, ConfigBuilder, LevelFilter};
use std::{fs::OpenOptions, error::Error};


pub fn init_logging() -> Result<(), Box<dyn Error>> {
    // 1. Open (or create) the log file in append mode
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("logfile.txt")?;

    // 2. Build a config, attempting to use local‐time offset if available
    let config = {
        let mut builder = ConfigBuilder::new();

        let _ = builder.set_time_offset_to_local();
        builder.build()
    };

    // 3. Initialize the logger at INFO level, writing into logfile.txt
    WriteLogger::init(LevelFilter::Info, config, log_file)?;

    Ok(())
}
