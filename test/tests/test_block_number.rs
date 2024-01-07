#![feature(assert_matches)]

use rpc_test::test_config::TestConfig;
use starknet::providers::{
    jsonrpc::{HttpTransport, JsonRpcClient},
    Provider};
use url::Url;

#[tokio::test]
async fn work_existing_block() {
    let config = TestConfig::new("./secret.json").expect("'./secret.json' must contain correct node urls");
    let deoxys = JsonRpcClient::new(HttpTransport::new(
        Url::parse(&config.deoxys).unwrap()
    ));
    let alchemy = JsonRpcClient::new(HttpTransport::new(
        Url::parse(&config.alchemy).unwrap()
    ));

    let response_deoxys = deoxys.block_number().await.expect("Deoxys : Error while getting the block number");
    let response_alchemy = alchemy.block_number().await.expect("RPC : Error while getting the block number");

    assert_eq!(response_deoxys, response_alchemy);
}