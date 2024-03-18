use std::collections::HashMap;

use macro_utils::TestConfig;
use rstest::fixture;
use starknet_providers::{jsonrpc::HttpTransport, JsonRpcClient};
use url::Url;

use crate::constants::*;
use crate::map;

#[fixture]
pub fn config() -> TestConfig {
    TestConfig::new("../secret.json").expect("'../secret.json' must contain correct node urls")
}

#[fixture]
pub fn deoxys(config: TestConfig) -> JsonRpcClient<HttpTransport> {
    JsonRpcClient::new(HttpTransport::new(
        Url::parse(&config.deoxys).expect("Error parsing Deoxys node url"),
    ))
}

#[fixture]
pub fn pathfinder(config: TestConfig) -> JsonRpcClient<HttpTransport> {
    JsonRpcClient::new(HttpTransport::new(
        Url::parse(&config.pathfinder).expect("Error parsing Pathfinder node url"),
    ))
}

#[fixture]
pub fn juno(config: TestConfig) -> JsonRpcClient<HttpTransport> {
    JsonRpcClient::new(HttpTransport::new(
        Url::parse(&config.juno).expect("Error parsing Juno node url"),
    ))
}

#[fixture]
pub fn clients(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
    juno: JsonRpcClient<HttpTransport>,
) -> HashMap<String, JsonRpcClient<HttpTransport>> {
    map! {
        String::from(DEOXYS) => deoxys,
        String::from(PATHFINDER) => pathfinder,
        String::from(JUNO) => juno,
    }
}
