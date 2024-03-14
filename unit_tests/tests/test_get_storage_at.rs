#![feature(assert_matches)]

mod common;
use common::*;

use std::collections::HashMap;

use starknet_core::types::{BlockId, BlockTag, FieldElement, StarknetError};
use starknet_providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider};

///
/// Unit test for `starknet_getStorageAt`
///
/// purpose: call getStorageAt on invalid block.
/// fail case: invalid block.
///
#[rstest]
#[tokio::test]
async fn fail_non_existing_block(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];

    let response_deoxys = deoxys
        .get_storage_at(
            FieldElement::from_hex_be(CONTRACT_ADDR).unwrap(),
            FieldElement::from_hex_be(CONTRACT_KEY).unwrap(),
            BlockId::Hash(FieldElement::ZERO),
        )
        .await;

    assert!(
        response_deoxys.is_err(),
        "Expected an error, but got a result"
    );

    if let Err(error) = response_deoxys {
        let is_correct_error = checking_error_format(
            &error,
            StarknetError::BlockNotFound,
        );

        assert!(
            is_correct_error,
            "Expected BlockNotFound error, but got a different error"
        );
    }
}

///
/// Unit test for `starknet_getStorageAt`
///
/// purpose: call getStorageAt on non-existing contract.
/// fail case: non-existing contract.
///
#[rstest]
#[tokio::test]
async fn fail_non_existing_contract(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];

    let response_deoxys = deoxys
        .get_storage_at(
            FieldElement::ZERO,
            FieldElement::from_hex_be(CONTRACT_KEY).unwrap(),
            BlockId::Tag(BlockTag::Latest),
        )
        .await;

    assert!(
        response_deoxys.is_err(),
        "Expected an error, but got a result"
    );

    if let Err(error) = response_deoxys {
        let is_correct_error = checking_error_format(
            &error,
            StarknetError::ContractNotFound,
        );

        assert!(
            is_correct_error,
            "Expected ContractNotFound error, but got a different error"
        );
    }
}

///
/// Unit test for `starknet_getStorageAt`
///
/// purpose: call getStorageAt with invalid storage key.
/// fail case: invalid storage key.
///
#[rstest]
#[tokio::test]
async fn fail_invalid_storage_key(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];

    let response_deoxys = deoxys
        .get_storage_at(
            FieldElement::from_hex_be(CONTRACT_ADDR).unwrap(),
            FieldElement::ZERO,
            BlockId::Tag(BlockTag::Latest),
        )
        .await
        .expect("Error waiting for response from Deoxys client");

    assert_eq!(response_deoxys, FieldElement::ZERO);
}

///
/// Unit test for `starknet_getStorageAt`
///
/// purpose: call getStorageAt with valid arguments.
/// success case: retrieve valid storage.
///
#[rstest]
#[tokio::test]
async fn work_get_storage(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    // TODO: get contract key from field name
    let response_deoxys = deoxys
        .get_storage_at(
            FieldElement::from_hex_be(CONTRACT_ADDR).unwrap(),
            FieldElement::from_hex_be(CONTRACT_KEY).unwrap(),
            BlockId::Tag(BlockTag::Latest),
        )
        .await
        .expect("Error waiting for response from Deoxys client");
    let response_pathfinder = pathfinder
        .get_storage_at(
            FieldElement::from_hex_be(CONTRACT_ADDR).unwrap(),
            FieldElement::from_hex_be(CONTRACT_KEY).unwrap(),
            BlockId::Tag(BlockTag::Latest),
        )
        .await
        .expect("Error waiting for response from Pathfinder client");

    assert_eq!(response_deoxys, response_pathfinder);
}
