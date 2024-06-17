#![feature(assert_matches)]

mod common;
use common::*;

use starknet_core::types::{
    BlockId, BlockTag, MaybePendingStateUpdate, StarknetError, StateUpdate,
};
use starknet_providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider};
use std::collections::HashMap;

pub fn extract_and_sort_state_update(maybe_update: MaybePendingStateUpdate) -> Option<StateUpdate> {
    match maybe_update {
        MaybePendingStateUpdate::Update(state_update) => Some(sort_state_update(state_update)),
        MaybePendingStateUpdate::PendingUpdate(_) => None, // or handle pending update if necessary
    }
}

pub fn sort_state_update(state_update: StateUpdate) -> StateUpdate {
    let mut sorted_state_update = state_update.clone();
    let state_diff = &mut sorted_state_update.state_diff;
    let storage_diffs = &mut state_diff.storage_diffs;

    for storage_diff in storage_diffs.iter_mut() {
        storage_diff.storage_entries.sort_by_key(|x| x.key);
    }

    storage_diffs.sort_by_key(|x| x.address);
    state_diff.deprecated_declared_classes.sort();
    state_diff.declared_classes.sort_by_key(|x| x.class_hash);
    state_diff.deployed_contracts.sort_by_key(|x| x.address);
    state_diff
        .replaced_classes
        .sort_by_key(|x| x.contract_address);
    state_diff.nonces.sort_by_key(|x| x.contract_address);

    sorted_state_update
}

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
    let deoxys = &clients[mainnet::network::DEOXYS];

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
async fn work_genesis_block(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[mainnet::network::DEOXYS];
    let pathfinder = &clients[mainnet::network::PATHFINDER];
    let juno = &clients[mainnet::network::JUNO];

    let block_number = BlockId::Number(0);

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

    let sorted_deoxys = extract_and_sort_state_update(response_deoxys);
    let sorted_pathfinder = extract_and_sort_state_update(response_pathfinder);
    let sorted_juno = extract_and_sort_state_update(response_juno);

    assert_eq!(
        sorted_deoxys, sorted_pathfinder,
        "The sorted responses do not match"
    );
    assert_eq!(
        sorted_deoxys, sorted_juno,
        "The sorted responses do not match"
    );
    assert_eq!(
        sorted_juno, sorted_pathfinder,
        "The sorted responses do not match"
    );
}

#[rstest]
#[tokio::test]
async fn work_existing_block(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[mainnet::network::DEOXYS];
    let pathfinder = &clients[mainnet::network::PATHFINDER];
    let juno = &clients[mainnet::network::JUNO];

    let block_number = BlockId::Number(250000);

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

    // Extract and sort the updates
    let sorted_deoxys = extract_and_sort_state_update(response_deoxys);
    let sorted_pathfinder = extract_and_sort_state_update(response_pathfinder);
    let sorted_juno = extract_and_sort_state_update(response_juno);

    assert_eq!(
        sorted_deoxys, sorted_pathfinder,
        "The sorted responses do not match"
    );
    assert_eq!(
        sorted_deoxys, sorted_juno,
        "The sorted responses do not match"
    );
    assert_eq!(
        sorted_juno, sorted_pathfinder,
        "The sorted responses do not match"
    );
}

#[rstest]
#[tokio::test]
async fn work_loop_existing_block(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[mainnet::network::DEOXYS];
    let pathfinder = &clients[mainnet::network::PATHFINDER];
    let juno = &clients[mainnet::network::JUNO];

    for i in 0..5 {
        let block_number = BlockId::Number(i * 10000);
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

        // Extract and sort the updates
        let sorted_deoxys = extract_and_sort_state_update(response_deoxys);
        let sorted_pathfinder = extract_and_sort_state_update(response_pathfinder);
        let sorted_juno = extract_and_sort_state_update(response_juno);

        assert_eq!(
            sorted_deoxys, sorted_pathfinder,
            "The sorted responses do not match"
        );
        assert_eq!(
            sorted_deoxys, sorted_juno,
            "The sorted responses do not match"
        );
        assert_eq!(
            sorted_juno, sorted_pathfinder,
            "The sorted responses do not match"
        );
    }
}

#[rstest]
#[tokio::test]
#[ignore = "Pending data is not supported yet"]
async fn work_block_pending(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[mainnet::network::DEOXYS];
    let pathfinder = &clients[mainnet::network::PATHFINDER];
    let juno = &clients[mainnet::network::JUNO];

    let block_number = BlockId::Tag(BlockTag::Pending);
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

    // Extract and sort the updates
    let sorted_deoxys = extract_and_sort_state_update(response_deoxys);
    let sorted_pathfinder = extract_and_sort_state_update(response_pathfinder);
    let sorted_juno = extract_and_sort_state_update(response_juno);

    assert_eq!(
        sorted_deoxys, sorted_pathfinder,
        "The sorted responses do not match"
    );
    assert_eq!(
        sorted_deoxys, sorted_juno,
        "The sorted responses do not match"
    );
    assert_eq!(
        sorted_juno, sorted_pathfinder,
        "The sorted responses do not match"
    );
}

#[rstest]
#[tokio::test]
async fn work_block_latest(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[mainnet::network::DEOXYS];
    let pathfinder = &clients[mainnet::network::PATHFINDER];
    let juno = &clients[mainnet::network::JUNO];

    let block_number = BlockId::Tag(BlockTag::Latest);
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

    // Extract and sort the updates
    let sorted_deoxys = extract_and_sort_state_update(response_deoxys);
    let sorted_pathfinder = extract_and_sort_state_update(response_pathfinder);
    let sorted_juno = extract_and_sort_state_update(response_juno);

    assert_eq!(
        sorted_deoxys, sorted_pathfinder,
        "The sorted responses do not match"
    );
    assert_eq!(
        sorted_deoxys, sorted_juno,
        "The sorted responses do not match"
    );
    assert_eq!(
        sorted_juno, sorted_pathfinder,
        "The sorted responses do not match"
    );
}
