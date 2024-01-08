use anyhow::Context;
use serde::Deserialize;
use serde_json::from_str;
use std::{fs::File, io::Read};

#[derive(PartialEq, Debug, Deserialize)]
pub struct TestConfig {
    pub pathfinder: String,
    pub deoxys: String,
}

impl TestConfig {
    pub fn new(path: &str) -> anyhow::Result<Self> {
        let mut file = File::open(path)?;
        let mut content = String::new();

        file.read_to_string(&mut content)?;

        let config: TestConfig = from_str(&content)
            .with_context(|| format!("Could not deserialize test at {path} into Config"))?;

        Ok(config)
    }
}