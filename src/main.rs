use dless::{dless, DlessConfig};
use std::process::ExitCode;

fn main() -> ExitCode {
    let config = DlessConfig {
        file: "testdata/dlog0.log".to_string(),
    };
    if dless(config) == 0 {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
