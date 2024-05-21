use clap::Parser;
use dless::{dless, DlessConfig};
use std::process::ExitCode;

fn main() -> ExitCode {
    let config = DlessConfig::parse();
    dless(&config).unwrap_or_else(|err| {
        eprintln!("Unexpected error: {} - {}", err, config.file);
        ExitCode::FAILURE
    })
}
