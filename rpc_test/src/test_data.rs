use anyhow::Context;
use serde::Deserialize;
use std::{fs::File, io::Read};

#[derive(Deserialize, Debug, PartialEq)]
pub struct TestData {
    pub cmd: String,
    pub arg: Vec<String>,
    pub block_range: Option<BlockRange>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct BlockRange {
    pub start_inclusive: u32,
    pub stop_inclusive: u32,
}

impl TestData {
    pub fn new(path: &str) -> anyhow::Result<Self> {
        let mut file = File::open(path)?;
        let mut content = String::new();

        file.read_to_string(&mut content)?;

        let test_data: TestData = serde_json::from_str(&content)
            .with_context(|| format!("Could not deserialize test at into TestData"))?;

        Ok(test_data)
    }
}