#![feature(assert_matches)]

mod common;
use std::{assert_matches::assert_matches, collections::HashMap};

use common::*;
use starknet_core::types::{BlockId, FieldElement, StarknetError};
use starknet_providers::{
    jsonrpc::HttpTransport, JsonRpcClient, Provider, ProviderError,
};

///
/// Unit test for `starknet_getTransactionByBlockIdAndIndex`
///
/// purpose: call on non-existent block.
/// fail case: invalid block
///
#[rstest]
#[tokio::test]
async fn fail_non_existent_block(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];

    let response_deoxys = deoxys
        .get_transaction_by_block_id_and_index(BlockId::Hash(FieldElement::ZERO), 0)
        .await
        .err();

    assert_matches!(
        response_deoxys,
        Some(ProviderError::StarknetError(StarknetError::BlockNotFound))
    );
}

///
/// Unit test for `starknet_getTransactionByBlockIdAndIndex`
///
/// purpose: call on valid block with out-of-range index.
/// fail case: index out of block range (block 5000 only has 389 transactions)
///
#[rstest]
#[tokio::test]
async fn fail_non_existent_block_index(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];

    let response_deoxys = deoxys
        .get_transaction_by_block_id_and_index(BlockId::Number(5000), 389)
        .await
        .err();

    assert_matches!(
        response_deoxys,
        Some(ProviderError::StarknetError(StarknetError::InvalidTransactionIndex))
    );
}

///
/// Unit test for `starknet_getTransactionByBlockIdAndIndex`
///
/// purpose: get INVOKE transaction.
/// success case: client retrieves same transaction with getTransactionByBlockIdAndIndex and getTransactionByHash.
///
#[rstest]
#[tokio::test]
async fn work_deploy_invoke(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    // getting transaction through block number and index
    let response_deoxys = deoxys
        .get_transaction_by_block_id_and_index(
            BlockId::Number(TRANSACTION_INVOKE_BLOCK_NB),
            TRANSACTION_INVOKE_INDEX,
        )
        .await
        .expect("Error waiting for response from Deoxys node");

    let response_pathfinder = pathfinder
        .get_transaction_by_block_id_and_index(
            BlockId::Number(TRANSACTION_INVOKE_BLOCK_NB),
            TRANSACTION_INVOKE_INDEX,
        )
        .await
        .expect("Error waiting for response from Pathfinder node");

    // getting transaction through hash
    let response_expected = deoxys
        .get_transaction_by_hash(FieldElement::from_hex_be(TRANSACTION_INVOKE).unwrap())
        .await
        .expect("Error waiting for response from Deoxys node");

    assert_eq!(response_deoxys, response_expected);
    assert_eq!(response_deoxys, response_pathfinder);
}

///
/// Unit test for `starknet_getTransactionByBlockIdAndIndex`
///
/// purpose: get L1_HANDLER transaction.
/// success case: client retrieves same transaction with getTransactionByBlockIdAndIndex and getTransactionByHash.
///
#[rstest]
#[tokio::test]
async fn work_deploy_l1_handler(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    // getting transaction through block number and index
    let response_deoxys = deoxys
        .get_transaction_by_block_id_and_index(
            BlockId::Number(TRANSACTION_L1_HANDLER_BLOCK_NB),
            TRANSACTION_L1_HANDLER_INDEX,
        )
        .await
        .expect("Error waiting for response from Deoxys node");

    let response_pathfinder = pathfinder
        .get_transaction_by_block_id_and_index(
            BlockId::Number(TRANSACTION_L1_HANDLER_BLOCK_NB),
            TRANSACTION_L1_HANDLER_INDEX,
        )
        .await
        .expect("Error waiting for response from Pathfinder node");

    // getting transaction through hash
    let response_expected = deoxys
        .get_transaction_by_hash(FieldElement::from_hex_be(TRANSACTION_L1_HANDLER).unwrap())
        .await
        .expect("Error waiting for response from Deoxys node");

    assert_eq!(response_deoxys, response_expected);
    assert_eq!(response_deoxys, response_pathfinder);
}

///
/// Unit test for `starknet_getTransactionByBlockIdAndIndex`
///
/// purpose: get DECLARE transaction.
/// success case: client retrieves same transaction with getTransactionByBlockIdAndIndex and getTransactionByHash.
///
#[rstest]
#[tokio::test]
async fn work_deploy_declare(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    // getting transaction through block number and index
    let response_deoxys = deoxys
        .get_transaction_by_block_id_and_index(
            BlockId::Number(TRANSACTION_DECLARE_BLOCK_NB),
            TRANSACTION_DECLARE_INDEX,
        )
        .await
        .expect("Error waiting for response from Deoxys node");

    let response_pathfinder = pathfinder
        .get_transaction_by_block_id_and_index(
            BlockId::Number(TRANSACTION_DECLARE_BLOCK_NB),
            TRANSACTION_DECLARE_INDEX,
        )
        .await
        .expect("Error waiting for response from Pathfinder node");

    // getting transaction through hash
    let response_expected = deoxys
        .get_transaction_by_hash(FieldElement::from_hex_be(TRANSACTION_DECLARE).unwrap())
        .await
        .expect("Error waiting for response from Deoxys node");

    assert_eq!(response_deoxys, response_expected);
    assert_eq!(response_deoxys, response_pathfinder);
}

///
/// Unit test for `starknet_getTransactionByBlockIdAndIndex`
///
/// purpose: get DEPLOY_ACCOUNT transaction.
/// success case: client retrieves same transaction with getTransactionByBlockIdAndIndex and getTransactionByHash.
///
#[rstest]
#[tokio::test]
async fn work_deploy_account(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    // getting transaction through block number and index
    let response_deoxys = deoxys
        .get_transaction_by_block_id_and_index(
            BlockId::Number(TRANSACTION_DEPLOY_ACCOUNT_BLOCK_NB),
            TRANSACTION_DEPLOY_ACCOUNT_INDEX,
        )
        .await
        .expect("Error waiting for response from Deoxys node");

    let response_pathfinder = pathfinder
        .get_transaction_by_block_id_and_index(
            BlockId::Number(TRANSACTION_DEPLOY_ACCOUNT_BLOCK_NB),
            TRANSACTION_DEPLOY_ACCOUNT_INDEX,
        )
        .await
        .expect("Error waiting for response from Pathfinder node");

    // getting transaction through hash
    let response_expected = deoxys
        .get_transaction_by_hash(FieldElement::from_hex_be(TRANSACTION_DEPLOY_ACCOUNT).unwrap())
        .await
        .expect("Error waiting for response from Deoxys node");

    assert_eq!(response_deoxys, response_expected);
    assert_eq!(response_deoxys, response_pathfinder);
}
