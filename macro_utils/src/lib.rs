use serde::Deserialize;
use starknet_providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider};
use std::{fs::File, io::Read};
use syn::{Expr, Lit};
use tokio::runtime;
use url::Url;

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

        let config: TestConfig = serde_json::from_str(&content)
            .expect("Could not deserialize test at {path} into Config");

        Ok(config)
    }
}

pub fn get_block_number() -> u64 {
    let config =
        TestConfig::new("./secret.json").expect("'./secret.json' must contain correct node urls");
    let deoxys = JsonRpcClient::new(HttpTransport::new(
        Url::parse(&config.deoxys).expect("Error parsing Deoxys node url"),
    ));

    let rt = runtime::Runtime::new().unwrap();

    rt.block_on(async { deoxys.block_number().await.unwrap() })
}

pub fn extract_u64_from_expr(expr: Expr) -> Result<u64, String> {
    match expr {
        Expr::Lit(expr_lit) => match expr_lit.lit {
            Lit::Int(lit_int) => lit_int
                .base10_parse::<u64>()
                .map_err(|_| "Failed to parse integer".to_string()),
            _ => Err("Not an integer literal".to_string()),
        },
        _ => Err("Not a literal expression".to_string()),
    }
}
