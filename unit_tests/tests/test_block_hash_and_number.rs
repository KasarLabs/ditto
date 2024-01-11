mod common;
use common::*;

use std::collections::HashMap;

use starknet_providers::{
    jsonrpc::{HttpTransport, JsonRpcClient},
    Provider,
};

///
/// Unit test for `starknet_BlockHashAndNumber`
///
/// purpose: get block hash and number on latest block.
/// success case: retrieves correct block hash and number.
///
#[rstest]
#[tokio::test]
async fn work_latest_block(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let response_deoxys = deoxys
        .block_hash_and_number()
        .await
        .expect("Error waiting for response from Deoxys node");
    let response_pathfinder = pathfinder
        .block_hash_and_number()
        .await
        .expect("Error waiting for response from Deoxys node");
    let response_expected = deoxys
        .block_number()
        .await
        .expect("Error waiting for response from Deoxys node");

    assert_eq!(response_deoxys.block_number, response_expected);
    assert_eq!(response_deoxys, response_pathfinder);
}
