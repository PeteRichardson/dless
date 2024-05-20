use clap::Parser;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser, Debug, Clone)]
#[command(version, about)]
pub struct DlessConfig {
    /// log file to view
    #[arg(default_value = "testdata/dlog0.log")]
    pub file: PathBuf,
}

pub fn dless(config: &DlessConfig) -> std::result::Result<ExitCode, Box<dyn std::error::Error>> {
    let file = File::open(&config.file)?;

    for line in BufReader::new(file).lines() {
        println!("{}", line?);
    }
    Ok(ExitCode::SUCCESS)
}
