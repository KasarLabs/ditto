use anyhow::anyhow;
use lazy_static::lazy_static;
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

#[derive(Clone, Debug)]
pub struct RpcData {
    pub latest_chain_block: u64,
    pub block_number: u64,
    pub spec_version: String,
}

lazy_static! {
    pub static ref RPC_DATA: RpcData = get_rpc_data();
}

fn get_rpc_data() -> RpcData {
    let config =
        TestConfig::new("./secret.json").expect("'./secret.json' must contain correct node urls");
    let deoxys = JsonRpcClient::new(HttpTransport::new(
        Url::parse(&config.deoxys).expect("Error parsing Deoxys node url"),
    ));

    let pathfinder = JsonRpcClient::new(HttpTransport::new(
        Url::parse(&config.pathfinder).expect("Error parsing Pathfinder node url"),
    ));

    let rt = runtime::Runtime::new().unwrap();

    rt.block_on(async {
        RpcData {
            latest_chain_block: pathfinder.block_number().await.unwrap(),
            block_number: deoxys.block_number().await.unwrap(),
            spec_version: deoxys.spec_version().await.unwrap(),
        }
    })
}

pub fn extract_expr_to_str(expr: &Expr) -> anyhow::Result<String> {
    match expr {
        Expr::Lit(expr_lit) => match &expr_lit.lit {
            Lit::Str(lit_str) => anyhow::Ok(lit_str.value()),
            _ => Err(anyhow!("Not a string literal")),
        },
        _ => Err(anyhow!("Not a literal expression")),
    }
}

pub fn extract_expr_to_u64(expr: &Expr) -> anyhow::Result<u64> {
    match expr {
        Expr::Lit(expr_lit) => match &expr_lit.lit {
            Lit::Int(lit_int) => match lit_int.base10_parse::<u64>() {
                Ok(n) => anyhow::Ok(n),
                Err(_) => Err(anyhow!("Failed to convert literal")),
            },
            _ => Err(anyhow!("Not an integer literal")),
        },
        _ => Err(anyhow!("Not a literal expression")),
    }
}
