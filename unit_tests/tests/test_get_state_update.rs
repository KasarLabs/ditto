#![feature(assert_matches)]

mod common;
use common::*;

use starknet_core::types::{BlockId, StarknetError};
use starknet_providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider};
use std::{collections::HashMap, os::macos::raw};

/// Test for the `get_state_update` Deoxys RPC method
/// # Arguments
// * `block_id` - The block id to get the state update from
//
// # Returns
// * `state update`
// or
// * `pending state update`
//
// # Errors
// * `block_not_found` - If the block is not found or invalid

#[rstest]
#[tokio::test]
async fn fail_non_existing_block(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];

    let response_deoxys = deoxys.get_state_update(BlockId::Number(0)).await;

    assert!(
        response_deoxys.is_ok(),
        "Expected an error, but got a result"
    );

    if let Err(error) = response_deoxys {
        let is_correct_error = checking_error_format(&error, StarknetError::BlockNotFound);

        assert!(
            is_correct_error,
            "Expected InvalidTransactionHash error, but got a different error"
        );
    }
}

#[rstest]
#[tokio::test]
async fn work_existing_block(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];
    let juno = &clients[JUNO];

    let block_number = BlockId::Number(10000);

    let response_deoxys = deoxys
        .get_state_update(block_number)
        .await
        .expect("Deoxys : Error while getting the state update");
    let response_pathfinder = pathfinder
        .get_state_update(block_number)
        .await
        .expect("RPC : Error while getting the state update");
    let response_juno = juno
        .get_state_update(block_number)
        .await
        .expect("RPC : Error while getting the state update");

    let raw_deoxys = format!("{:?}", response_deoxys);
    let raw_pathfinder = format!("{:?}", response_pathfinder);
    let raw_juno = format!("{:?}", response_juno);

    let mut sorted_deoxys: Vec<char> = raw_deoxys.chars().collect();
    sorted_deoxys.sort();  // Use sort instead of sort_unstable
    let sorted_deoxys: String = sorted_deoxys.into_iter().collect();

    let mut sorted_pathfinder: Vec<char> = raw_pathfinder.chars().collect();
    sorted_pathfinder.sort();  // Use sort instead of sort_unstable
    let sorted_pathfinder: String = sorted_pathfinder.into_iter().collect();

    let mut sorted_juno: Vec<char> = raw_juno.chars().collect();
    sorted_juno.sort();  // Use sort instead of sort_unstable
    let sorted_juno: String = sorted_juno.into_iter().collect();

    assert_eq!(sorted_deoxys, sorted_pathfinder, "The sorted responses do not match");
    assert_eq!(sorted_deoxys, sorted_juno, "The sorted responses do not match");
    assert_eq!(sorted_juno, sorted_pathfinder, "The sorted responses do not match");
}

