use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct TestData {
    pub tests: Vec<Unit>
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Unit {
    pub cmd: String,
    pub arg: Vec<String>,
    pub block_range: Option<BlockRange>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, Copy)]
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