use clap::Parser;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct DlessConfig {
    /// log file to view
    #[arg(default_value = "testdata/dlog0.log")]
    pub file: PathBuf,
}

pub fn dless(config: DlessConfig) -> ExitCode {
    println!("Hello, {:}!", config.file.display());
    ExitCode::SUCCESS
}
