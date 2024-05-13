#[derive(Debug)]
pub struct DlessConfig {
    pub file: String,
}

pub fn dless(config: DlessConfig) -> usize {
    println!("Hello, {}!", config.file);
    0
}
