#![feature(assert_matches)]

mod common;
use common::*;

use starknet_core::types::{BlockId, EthAddress, FieldElement, MsgFromL1, StarknetError};
use starknet_providers::{
    jsonrpc::{HttpTransport, JsonRpcClient},
    Provider, ProviderError,
};
use std::assert_matches::assert_matches;
use std::collections::HashMap;

/// Test for the `get_state_update` Deoxys RPC method
/// # Arguments
// * `message` - A message from the L1 containg a from address (Ethereum address), a to address (Starknet address),
//                      an entry point selector (hex string) and a payload (list of field elements)
// * `block_id` - The block id to get the state update from
//
// # Returns
// * `fee estimation` - The fee estimation for the given message
//
// # Errors
// * `block_not_found` - If the block is not found or invalid
// * `contract_not_found` - If the contract is not found or invalid
// * `contract_error` - If the contract is found but the message is invalid

pub fn get_message_from_l1(
    from: &str,
    to: FieldElement,
    selector: &str,
    load: &Vec<FieldElement>,
) -> MsgFromL1 {
    let loading = load.to_owned();
    MsgFromL1 {
        from_address: EthAddress::from_hex(from).unwrap(),
        to_address: to,
        entry_point_selector: FieldElement::from_hex_be(selector).unwrap(),
        payload: loading,
    }
}

#[rstest]
#[tokio::test]
#[ignore = "Need to fix unwrap on error due to empty constants"]
async fn fail_non_existing_block(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];

    let payload_message: Vec<FieldElement> = vec![]; //TODO: Fill this with a valid payload message [ACCOUNT, METHOD, ...]
    let contract_address = FieldElement::from_hex_be(TEST_CONTRACT_ADDRESS).unwrap();
    let message = get_message_from_l1(
        ETHEREUM_ADDRESS,
        contract_address,
        SELECTOR_NAME,
        &payload_message,
    );

    let deoxys_message_fee = deoxys
        .estimate_message_fee(message, BlockId::Hash(FieldElement::ZERO))
        .await;
    assert_matches!(
        deoxys_message_fee,
        Err(ProviderError::StarknetError(StarknetError::BlockNotFound))
    );
}

#[rstest]
#[tokio::test]
#[ignore = "Need to fix unwrap on error due to empty constants"]
async fn fail_contract_not_found(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];

    let unknown_contract_address =
        FieldElement::from_hex_be("0x4269DEADBEEF").expect("Invalid Contract Address");
    let payload_message: Vec<FieldElement> = vec![]; //TODO: Fill this with a valid payload message [ACCOUNT, METHOD, ...]
    let message = get_message_from_l1(
        ETHEREUM_ADDRESS,
        unknown_contract_address,
        SELECTOR_NAME,
        &payload_message,
    );

    let deoxys_message_fee = deoxys
        .estimate_message_fee(message, BlockId::Hash(FieldElement::ZERO))
        .await;
    assert_matches!(
        deoxys_message_fee,
        Err(ProviderError::StarknetError(
            StarknetError::ContractNotFound
        ))
    )
}

#[rstest]
#[tokio::test]
#[ignore = "Need to fix unwrap on error due to empty constants"]
async fn fail_contract_error(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];

    //On this test, the contract address must be valid,
    //but the from_address, entry_point_selector or the payload must be invalid
    let payload_message: Vec<FieldElement> = vec![]; //TODO: Fill this with a valid payload message [ACCOUNT, METHOD, ...]
    let contract_address = FieldElement::from_hex_be(TEST_CONTRACT_ADDRESS).unwrap();
    let message = get_message_from_l1(
        INVALID_ETHEREUM_ADDRESS,
        contract_address,
        SELECTOR_NAME,
        &payload_message,
    );

    let deoxys_message_fee = deoxys
        .estimate_message_fee(message, BlockId::Hash(FieldElement::ZERO))
        .await;
    assert_matches!(
        deoxys_message_fee,
        Err(ProviderError::StarknetError(StarknetError::ContractError(
            _
        )))
    )
}

#[rstest]
#[tokio::test]
#[ignore = "Need to fix unwrap on error due to empty constants"]
async fn estimate_message_fee_works_ok(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let payload_message: Vec<FieldElement> = vec![]; //TODO: Fill this with a valid payload message [ACCOUNT, METHOD, ...]
    let contract_address = FieldElement::from_hex_be(TEST_CONTRACT_ADDRESS).unwrap();
    let deoxys_message = get_message_from_l1(
        ETHEREUM_ADDRESS,
        contract_address,
        SELECTOR_NAME,
        &payload_message,
    );
    let pathfinder_message = get_message_from_l1(
        ETHEREUM_ADDRESS,
        contract_address,
        SELECTOR_NAME,
        &payload_message,
    );

    let deoxys_message_fee = deoxys
        .estimate_message_fee(deoxys_message, BlockId::Hash(FieldElement::ZERO))
        .await
        .unwrap();
    let pathfinder_message_fee = pathfinder
        .estimate_message_fee(pathfinder_message, BlockId::Hash(FieldElement::ZERO))
        .await
        .unwrap();
    assert_eq!(deoxys_message_fee, pathfinder_message_fee);
}
