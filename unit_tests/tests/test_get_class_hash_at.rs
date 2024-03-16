#![feature(assert_matches)]

mod common;
use common::*;

use std::collections::HashMap;

use starknet_core::types::{BlockId, BlockTag, FieldElement, StarknetError};
use starknet_providers::{
    jsonrpc::{HttpTransport, JsonRpcError},
    JsonRpcClient, Provider,
};

///
/// Unit test for `starknet_getClassHashAt`
///
/// purpose: call getClassHashAt on invalid block.
/// fail case: invalid block hash.
///
#[rstest]
#[tokio::test]
async fn fail_non_existing_block(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];

    let response_deoxys = deoxys
        .get_class_hash_at(
            BlockId::Hash(FieldElement::ZERO),
            FieldElement::from_hex_be(STARKGATE_ETH_CONTRACT_ADDR).unwrap(),
        )
        .await
        .err();

    assert!(
        response_deoxys.is_some(),
        "Expected an error, but got a result"
    );

    let is_correct_error = checking_error_format(
        response_deoxys.as_ref().unwrap(),
        StarknetError::BlockNotFound,
    );

    assert!(
        is_correct_error,
        "Expected BlockNotFound error, but got a different error"
    );
}

///
/// Unit test for `starknet_getClassHashAt`
///
/// purpose: call getClassHashAt on non-existent contract.
/// fail case: invalid contract hash.
///
#[rstest]
#[tokio::test]
async fn fail_non_existing_contract(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];

    let response_deoxys = deoxys
        .get_class_hash_at(
            BlockId::Tag(BlockTag::Latest),
            FieldElement::from_hex_be(INVALID_CONTRACT_ADDR).unwrap(),
        )
        .await;

    let expected_error = JsonRpcError {
        code: -32602,
        message: "Invalid params".to_string(),
        data: None,
    };

    assert!(
        response_deoxys.is_err(),
        "Expected an error response, but got result. Expected error: {:?}",
        expected_error
    );
}

///
/// Unit test for `starknet_getClassHashAt`
///
/// purpose: call getClassHashAt on latest block.
/// success case: retrieve valid class hash.
///
#[rstest]
#[tokio::test]
async fn work_block_latest(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let class_hash_deoxys = deoxys
        .get_class_hash_at(
            BlockId::Tag(BlockTag::Latest),
            FieldElement::from_hex_be(STARKGATE_ETH_CONTRACT_ADDR).unwrap(),
        )
        .await
        .expect("Error waiting for response from Deoxys node");
    let class_hash_pathfinder = pathfinder
        .get_class_hash_at(
            BlockId::Tag(BlockTag::Latest),
            FieldElement::from_hex_be(STARKGATE_ETH_CONTRACT_ADDR).unwrap(),
        )
        .await
        .expect("Error waiting for response from Pathfinder node");

    assert_eq!(class_hash_deoxys, class_hash_pathfinder);
}

///
/// Unit test for `starknet_getClassHashAt`
///
/// purpose: call getClassHashAt on pending block.
/// success case: retrieve valid class hash.
///
#[rstest]
#[tokio::test]
#[ignore = "Pending fails some times when called on the cusp of being accepted, need virtual sequencer"]
async fn work_block_pending(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let class_hash_deoxys = deoxys
        .get_class_hash_at(
            BlockId::Tag(BlockTag::Pending),
            FieldElement::from_hex_be(STARKGATE_ETH_CONTRACT_ADDR).unwrap(),
        )
        .await
        .expect("Error waiting for response from Deoxys node");
    let class_hash_pathfinder = pathfinder
        .get_class_hash_at(
            BlockId::Tag(BlockTag::Pending),
            FieldElement::from_hex_be(STARKGATE_ETH_CONTRACT_ADDR).unwrap(),
        )
        .await
        .expect("Error waiting for response from Pathfinder node");

    assert_eq!(class_hash_deoxys, class_hash_pathfinder);
}
