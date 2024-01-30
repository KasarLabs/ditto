#![feature(assert_matches)]

mod common;
use std::{assert_matches::assert_matches, collections::HashMap};

use common::*;
use starknet_core::types::{FieldElement, StarknetError, Transaction};
use starknet_providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider, ProviderError};

///
/// Unit test for `starknet_getTransactionByHash`
///
/// purpose: call getTransactionHash on non existent transaction.
/// fail case: transaction does not exist.
///
#[require(spec_version = "0.5.1")]
#[rstest]
#[tokio::test]
async fn fail_non_existing_transaction(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];

    let response_deoxys = deoxys
        .get_transaction_by_hash(FieldElement::ZERO)
        .await
        .err();

    assert_matches!(
        response_deoxys,
        Some(ProviderError::StarknetError(
            StarknetError::TransactionHashNotFound
        ))
    );
}

///
/// Unit test for `starknet_getTransactionByHash`
///
/// purpose: call getTransactionHash on INVOKE transaction.
/// success case: retrieve correct INVOKE transaction.
///
#[require(block_min = 50_000, spec_version = "0.5.1")]
#[rstest]
#[tokio::test]
async fn work_transaction_invoke(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let response_deoxys = deoxys
        .get_transaction_by_hash(FieldElement::from_hex_be(TRANSACTION_INVOKE).unwrap())
        .await
        .expect("Error waiting for response from Deoxys node");

    let response_pathfinder = pathfinder
        .get_transaction_by_hash(FieldElement::from_hex_be(TRANSACTION_INVOKE).unwrap())
        .await
        .expect("Error waiting for response from Pathfinder node");

    assert_matches!(response_deoxys, Transaction::Invoke(_));
    assert_eq!(response_deoxys, response_pathfinder);
}

///
/// Unit test for `starknet_getTransactionByHash`
///
/// purpose: call getTransactionHash on L1_HANDLER transaction.
/// success case: retrieve correct INVOKE transaction.
///
#[require(block_min = 50_000, spec_version = "0.5.1")]
#[rstest]
#[tokio::test]
async fn work_transaction_l1_handler(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let response_deoxys = deoxys
        .get_transaction_by_hash(FieldElement::from_hex_be(TRANSACTION_L1_HANDLER).unwrap())
        .await
        .expect("Error waiting for response from Deoxys node");

    let response_pathfinder = pathfinder
        .get_transaction_by_hash(FieldElement::from_hex_be(TRANSACTION_L1_HANDLER).unwrap())
        .await
        .expect("Error waiting for response from Pathfinder node");

    assert_matches!(response_deoxys, Transaction::L1Handler(_));
    assert_eq!(response_deoxys, response_pathfinder);
}

///
/// Unit test for `starknet_getTransactionByHash`
///
/// purpose: call getTransactionHash on DECLARE transaction.
/// success case: retrieve correct DECLARE transaction.
///
#[require(block_min = 49_990spec_version = "0.5.1")]
#[rstest]
#[tokio::test]
async fn work_transaction_declare(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let response_deoxys = deoxys
        .get_transaction_by_hash(FieldElement::from_hex_be(TRANSACTION_DECLARE).unwrap())
        .await
        .expect("Error waiting for response from Deoxys node");

    let response_pathfinder = pathfinder
        .get_transaction_by_hash(FieldElement::from_hex_be(TRANSACTION_DECLARE).unwrap())
        .await
        .expect("Error waiting for response from Pathfinder node");

    assert_matches!(response_deoxys, Transaction::Declare(_));
    assert_eq!(response_deoxys, response_pathfinder);
}

// TODO: add tests for DEPLOY transaction

///
/// Unit test for `starknet_getTransactionByHash`
///
/// purpose: call getTransactionHash on DEPLOY_ACCOUNT transaction.
/// success case: retrieve correct DEPLOY_ACCOUNT transaction.
///
#[require(block_min = 50_000, spec_version = "0.5.1")]
#[rstest]
#[tokio::test]
async fn work_transaction_deploy_account(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let response_deoxys = deoxys
        .get_transaction_by_hash(FieldElement::from_hex_be(TRANSACTION_DEPLOY_ACCOUNT).unwrap())
        .await
        .expect("Error waiting for response from Deoxys node");

    let response_pathfinder = pathfinder
        .get_transaction_by_hash(FieldElement::from_hex_be(TRANSACTION_DEPLOY_ACCOUNT).unwrap())
        .await
        .expect("Error waiting for response from Pathfinder node");

    assert_matches!(response_deoxys, Transaction::DeployAccount(_));
    assert_eq!(response_deoxys, response_pathfinder);
}

/// helper function for testing transaction by hash
async fn work_with_hash(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
    transaction_hash: &str,
) {
    let transaction_hash =
        FieldElement::from_hex_be(transaction_hash).expect("Error parsing transaction hash");

    let response_deoxys = deoxys
        .get_transaction_by_hash(transaction_hash)
        .await
        .expect("Error waiting for response from Deoxys node");

    let response_pathfinder = pathfinder
        .get_transaction_by_hash(transaction_hash)
        .await
        .expect("Error waiting for response from Pathfinder node");

    assert_eq!(response_deoxys, response_pathfinder);
}

/// first transaction on block 0
#[require(spec_version = "0.5.1")]
#[rstest]
#[tokio::test]
async fn work_with_first_transaction_block_0(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
) {
    work_with_hash(
        deoxys,
        pathfinder,
        "0xe0a2e45a80bb827967e096bcf58874f6c01c191e0a0530624cba66a508ae75",
    )
    .await;
}

/// deploy transaction on block 0
#[require(spec_version = "0.5.1")]
#[rstest]
#[tokio::test]
async fn work_with_deploy_transaction_block_0(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
) {
    work_with_hash(
        deoxys,
        pathfinder,
        "0x12c96ae3c050771689eb261c9bf78fac2580708c7f1f3d69a9647d8be59f1e1",
    )
    .await;
}

/// invoke transaction on block 0
#[require(spec_version = "0.5.1")]
#[rstest]
#[tokio::test]
async fn work_with_invoke_transaction_block_0(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
) {
    work_with_hash(
        deoxys,
        pathfinder,
        "0xce54bbc5647e1c1ea4276c01a708523f740db0ff5474c77734f73beec2624",
    )
    .await;
}

/// deploy transaction on block 1
#[require(block_min = 1, spec_version = "0.5.1")]
#[rstest]
#[tokio::test]
async fn work_with_deploy_transaction_block_1(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
) {
    work_with_hash(
        deoxys,
        pathfinder,
        "0x2f07a65f9f7a6445b2a0b1fb90ef12f5fd3b94128d06a67712efd3b2f163533",
    )
    .await;
}

/// invoke transaction on block 0
#[require(block_min = 10, spec_version = "0.5.1")]
#[rstest]
#[tokio::test]
async fn work_with_invoke_transaction_block_10(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
) {
    work_with_hash(
        deoxys,
        pathfinder,
        "0x50398c6ec05a07642e5bd52c656e1650f3b057361283ecbb19d4062199e4626",
    )
    .await;
}

/// deploy transaction on block 10
#[require(block_min = 10, spec_version = "0.5.1")]
#[rstest]
#[tokio::test]
async fn work_with_deploy_transaction_block_10(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
) {
    work_with_hash(
        deoxys,
        pathfinder,
        "0x6eac388fc0a464285ea3c7ca79ddff73217b5466e97ac5415cf6548934dce82",
    )
    .await;
}
