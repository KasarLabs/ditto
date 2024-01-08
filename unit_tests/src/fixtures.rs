use std::collections::HashMap;

use rpc_test::test_config::TestConfig;
use rstest::fixture;
use starknet_providers::{JsonRpcClient, jsonrpc::HttpTransport};
use url::Url;

use crate::constants::*;
use crate::map;

#[fixture]
pub fn clients() -> HashMap<String, JsonRpcClient<HttpTransport>> {
    let config = TestConfig::new("./secret.json").expect("'./secret.json' must contain correct node urls");
    let deoxys = JsonRpcClient::new(HttpTransport::new(
        Url::parse(&config.deoxys).expect("Error parsing Deoxys node url")
    ));
    let pathfinder = JsonRpcClient::new(HttpTransport::new(
        Url::parse(&config.pathfinder).expect("Error parsing Deoxys node url")
    ));

    map!{
        String::from(DEOXYS) => deoxys,
        String::from(PATHFINDER) => pathfinder,
    }
}