use std::collections::HashMap;

use rstest::fixture;
use starknet_providers::{JsonRpcClient, jsonrpc::HttpTransport};
use url::Url;

use crate::TestConfig;
use crate::constants::*;
use crate::map;

#[fixture]
pub fn config() -> TestConfig {
    TestConfig::new("./secret.json").expect("'./secret.json' must contain correct node urls")
}

#[fixture]
pub fn deoxys(config: TestConfig) -> JsonRpcClient<HttpTransport> {
    JsonRpcClient::new(HttpTransport::new(
        Url::parse(&config.deoxys).expect("Error parsing Deoxys node url")
    ))
}

#[fixture]
pub fn pathfinder(config: TestConfig) -> JsonRpcClient<HttpTransport> {
    JsonRpcClient::new(HttpTransport::new(
        Url::parse(&config.pathfinder).expect("Error parsing Deoxys node url")
    ))
}

#[fixture]
pub fn clients(deoxys: JsonRpcClient<HttpTransport>, pathfinder: JsonRpcClient<HttpTransport>) -> HashMap<String, JsonRpcClient<HttpTransport>> {
    map!{
        String::from(DEOXYS) => deoxys,
        String::from(PATHFINDER) => pathfinder,
    }
}