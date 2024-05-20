use clap::Parser;
use dless::{dless, DlessConfig, DlessError};
use std::process::ExitCode;

fn main() -> ExitCode {
    let config = DlessConfig::parse();
    match dless(config) {
        Ok(()) => ExitCode::SUCCESS,
        Err(DlessError) => ExitCode::FAILURE,
    }
}
