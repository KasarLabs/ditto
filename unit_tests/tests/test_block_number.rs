mod common;
use common::*;

use std::collections::HashMap;

use starknet_providers::{jsonrpc::{HttpTransport, JsonRpcClient}, Provider};

#[rstest]
#[tokio::test]
async fn work_existing_block(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let response_deoxys = deoxys.block_number().await.expect("Deoxys : Error while getting the block number");
    let response_pathfinder = pathfinder.block_number().await.expect("RPC : Error while getting the block number");

    assert_eq!(response_deoxys, response_pathfinder);
}