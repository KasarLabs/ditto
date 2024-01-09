#![feature(assert_matches)]

mod common;
use std::collections::HashMap;

use common::*;
use starknet_core::types::{FieldElement, BlockId};
use starknet_providers::{JsonRpcClient, jsonrpc::HttpTransport, Provider};

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
    let response_deoxys = deoxys.get_transaction_by_block_id_and_index(
        BlockId::Number(TRANSACTION_INVOKE_BLOCK_NB),
        TRANSACTION_INVOKE_INDEX
    ).await.expect("Error waiting for response from Deoxys node");

    let response_pathfinder = pathfinder.get_transaction_by_block_id_and_index(
        BlockId::Number(TRANSACTION_INVOKE_BLOCK_NB),
        TRANSACTION_INVOKE_INDEX
    ).await.expect("Error waiting for response from Pathfinder node");

    // getting transaction through hash
    let response_expected = deoxys.get_transaction_by_hash(
        FieldElement::from_hex_be(TRANSACTION_INVOKE).unwrap()
    ).await.expect("Error waiting for response from Deoxys node");

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
    let response_deoxys = deoxys.get_transaction_by_block_id_and_index(
        BlockId::Number(TRANSACTION_DEPLOY_ACCOUNT_BLOCK_NB),
        TRANSACTION_DEPLOY_ACCOUNT_INDEX
    ).await.expect("Error waiting for response from Deoxys node");

    let response_pathfinder = pathfinder.get_transaction_by_block_id_and_index(
        BlockId::Number(TRANSACTION_DEPLOY_ACCOUNT_BLOCK_NB),
        TRANSACTION_DEPLOY_ACCOUNT_INDEX
    ).await.expect("Error waiting for response from Pathfinder node");

    // getting transaction through hash
    let response_expected = deoxys.get_transaction_by_hash(
        FieldElement::from_hex_be(TRANSACTION_DEPLOY_ACCOUNT).unwrap()
    ).await.expect("Error waiting for response from Deoxys node");

    assert_eq!(response_deoxys, response_expected);
    assert_eq!(response_deoxys, response_pathfinder);
}