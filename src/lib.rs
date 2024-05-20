use clap::Parser;
use std::fmt;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct DlessConfig {
    /// log file to view
    #[arg(default_value = "testdata/dlog0.log")]
    pub file: PathBuf,
}

type Result = std::result::Result<(), DlessError>;

#[derive(Debug, Clone)]
pub struct DlessError;

impl fmt::Display for DlessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error in Dless")
    }
}

pub fn dless(config: DlessConfig) -> Result {
    println!("Hello, {:}!", config.file.display());
    Ok(())
}
