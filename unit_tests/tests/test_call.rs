#![feature(assert_matches)]

mod common;
use std::collections::HashMap;

use common::*;
use starknet_core::{types::{FunctionCall, BlockTag, BlockId, FieldElement}, utils::get_selector_from_name};
use starknet_providers::{JsonRpcClient, jsonrpc::HttpTransport, Provider};

#[rstest]
#[tokio::test]
async fn work_correct_call(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let response_deoxys = deoxys.call(
        FunctionCall { 
            contract_address: FieldElement::from_hex_be(STARKGATE_ETH_BRIDGE_ADDR).unwrap(),
            entry_point_selector: get_selector_from_name("name").unwrap(), 
            calldata: vec![]
        },
        BlockId::Tag(BlockTag::Latest)
    ).await.expect("Error waiting for response from Deoxys node");

    let response_pathfinder = pathfinder.call(
        FunctionCall { 
            contract_address: FieldElement::from_hex_be(STARKGATE_ETH_BRIDGE_ADDR).unwrap(), 
            entry_point_selector: get_selector_from_name("name").unwrap(), 
            calldata: vec![]
        },
        BlockId::Tag(BlockTag::Latest)
    ).await.expect("Error waiting for response from Pathfinder node");

    assert_eq!(response_deoxys, response_pathfinder);
}