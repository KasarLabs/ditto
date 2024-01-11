#![feature(assert_matches)]

mod common;
use common::*;

use std::{assert_matches::assert_matches, collections::HashMap};

use starknet_core::types::{FieldElement, StarknetError};
use starknet_providers::{
    jsonrpc::{HttpTransport, JsonRpcClient},
    MaybeUnknownErrorCode, Provider, ProviderError, StarknetErrorWithMessage,
};

// invalid transaction_hash
#[rstest]
#[tokio::test]
async fn fail_invalid_transaction_hash(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];

    let response_deoxys = deoxys
        .get_transaction_receipt(FieldElement::ZERO)
        .await
        .err();

    assert_matches!(
        response_deoxys,
        Some(ProviderError::StarknetError(StarknetErrorWithMessage {
            message: _,
            code: MaybeUnknownErrorCode::Known(StarknetError::TransactionHashNotFound)
        }))
    );
}

/// reverted transaction on block 200000
#[rstest]
#[tokio::test]
async fn work_with_rejected_transaction(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let transaction_hash = FieldElement::from_hex_be(
        "0x410e4d74a2322b78d2e342ac376ea555c89b1a0fe73bb36067eb149da123dd1",
    )
    .expect("Error parsing transaction hash");

    let response_deoxys = deoxys
        .get_transaction_receipt(transaction_hash)
        .await
        .expect("Error waiting for response from Deoxys node");

    let response_pathfinder = pathfinder
        .get_transaction_receipt(transaction_hash)
        .await
        .expect("Error waiting for response from Pathfinder node");

    assert_eq!(response_deoxys, response_pathfinder);
}

/// first transaction on block 0
#[rstest]
#[tokio::test]
async fn work_with_first_transaction(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let transaction_hash = FieldElement::from_hex_be(
        "0xe0a2e45a80bb827967e096bcf58874f6c01c191e0a0530624cba66a508ae75",
    )
    .expect("Error parsing transaction hash");

    let response_deoxys = deoxys
        .get_transaction_receipt(transaction_hash)
        .await
        .expect("Error waiting for response from Deoxys node");

    let response_pathfinder = pathfinder
        .get_transaction_receipt(transaction_hash)
        .await
        .expect("Error waiting for response from Pathfinder node");

    assert_eq!(response_deoxys, response_pathfinder);
}

/// deploy transaction
#[rstest]
#[tokio::test]
async fn work_with_deploy(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let transaction_hash = FieldElement::from_hex_be(
        "0x12c96ae3c050771689eb261c9bf78fac2580708c7f1f3d69a9647d8be59f1e1",
    )
    .expect("Error parsing transaction hash");

    let response_deoxys = deoxys
        .get_transaction_receipt(transaction_hash)
        .await
        .expect("Error waiting for response from Deoxys node");

    let response_pathfinder = pathfinder
        .get_transaction_receipt(transaction_hash)
        .await
        .expect("Error waiting for response from Pathfinder node");

    assert_eq!(response_deoxys, response_pathfinder);
}

///invoke transaction
#[rstest]
#[tokio::test]
async fn work_with_invoke(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let transaction_hash = FieldElement::from_hex_be(
        "0xce54bbc5647e1c1ea4276c01a708523f740db0ff5474c77734f73beec2624",
    )
    .expect("Error parsing transaction hash");

    let response_deoxys = deoxys
        .get_transaction_receipt(transaction_hash)
        .await
        .expect("Error waiting for response from Deoxys node");

    let response_pathfinder = pathfinder
        .get_transaction_receipt(transaction_hash)
        .await
        .expect("Error waiting for response from Pathfinder node");

    assert_eq!(response_deoxys, response_pathfinder);
}