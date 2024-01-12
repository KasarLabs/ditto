mod common;
use common::*;

use starknet_providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider};
use std::collections::HashMap;

///
/// Unit test for `starknet_chainId`
///
/// purpose: get currently configured Starknet chain id
/// success case: retrieve correct chain id
///
#[rstest]
#[tokio::test]
async fn chain_id(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let response_deoxys = deoxys
        .chain_id()
        .await
        .expect("Error while getting chain id from deoxys");

    let response_pathfinder = pathfinder
        .chain_id()
        .await
        .expect("Error while getting chain id from pathfinder node");

    assert_eq!(response_deoxys, response_pathfinder);
}
