#![feature(assert_matches)]

mod common;
use std::{assert_matches::assert_matches, collections::HashMap};

use common::*;
use starknet_core::types::{FieldElement, StarknetError, Transaction};
use starknet_providers::{
    jsonrpc::HttpTransport, JsonRpcClient, MaybeUnknownErrorCode, Provider, ProviderError,
    StarknetErrorWithMessage,
};

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
        Some(ProviderError::StarknetError(StarknetErrorWithMessage {
            message: _,
            code: MaybeUnknownErrorCode::Known(StarknetError::TransactionHashNotFound)
        }))
    );
}

///
/// Unit test for `starknet_getTransactionByHash`
///
/// purpose: call getTransactionHash on INVOKE transaction.
/// success case: retrieve correct INVOKE transaction.
///
#[require(spec_version = "0.5.1")]
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
#[require(spec_version = "0.5.1")]
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
#[require(spec_version = "0.5.1")]
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
#[require(spec_version = "0.5.1")]
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
