#![feature(assert_matches)]

mod common;
use std::{collections::HashMap, assert_matches::assert_matches};

use common::*;
use starknet::macros::short_string;
use starknet_core::{types::{FunctionCall, BlockTag, BlockId, FieldElement, StarknetError}, utils::get_selector_from_name};
use starknet_providers::{JsonRpcClient, jsonrpc::HttpTransport, Provider, ProviderError, StarknetErrorWithMessage, MaybeUnknownErrorCode};

/**
 * Unit test for `starknet_call`
 * 
 * purpose: function request `name` to StarkGate ETH bridge contract
 * fail case: invalid block
 */
#[rstest]
#[tokio::test]
async fn fail_non_existing_block(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];

    let response_deoxys = deoxys.call(
        FunctionCall {
            contract_address: FieldElement::from_hex_be(STARKGATE_ETH_BRIDGE_ADDR).unwrap(),
            entry_point_selector: get_selector_from_name("name").unwrap(),
            calldata: vec![]
        },
        BlockId::Hash(FieldElement::ZERO)
    ).await.err();

    assert_matches!(
        response_deoxys,
        Some(ProviderError::StarknetError(StarknetErrorWithMessage {
            message: _,
            code: MaybeUnknownErrorCode::Known(StarknetError::BlockNotFound)
        }))
    );
}

/**
 * Unit test for `starknet_call`
 * 
 * purpose: function request `name` to StarkGate ETH bridge contract
 * fail case: invalid contract address
 */
#[rstest]
#[tokio::test]
async fn fail_non_existing_contract(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];

    let response_deoxys = deoxys.call(
        FunctionCall {
            contract_address: FieldElement::ZERO,
            entry_point_selector: get_selector_from_name("name").unwrap(),
            calldata: vec![]
        },
        BlockId::Tag(BlockTag::Latest)
    ).await.err();

    assert_matches!(
        response_deoxys,
        Some(ProviderError::StarknetError(StarknetErrorWithMessage {
            message: _,
            code: MaybeUnknownErrorCode::Known(StarknetError::ContractNotFound)
        }))
    );
}

/**
 * Unit test for `starknet_call`
 * 
 * purpose: function request `name` to StarkGate ETH bridge contract
 * fail case: invalid field element
 */
#[rstest]
#[tokio::test]
async fn fail_invalid_contract_entry_point_selector(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];

    let response_deoxys = deoxys.call(
        FunctionCall {
            contract_address: FieldElement::from_hex_be(STARKGATE_ETH_BRIDGE_ADDR).unwrap(),
            entry_point_selector: FieldElement::ZERO,
            calldata: vec![]
        },
        BlockId::Tag(BlockTag::Latest)
    ).await.err();

    assert_matches!(
        response_deoxys,
        Some(ProviderError::StarknetError(StarknetErrorWithMessage {
            message: _,
            code: MaybeUnknownErrorCode::Known(StarknetError::ContractError)
        }))
    );
}

/**
 * Unit test for `starknet_call`
 * 
 * purpose: function request `balanceOf` to StarkGate ETH bridge contract
 * fail case: missing call data. This is different from solely *invalid* call data, as we will see shortly
 */
#[rstest]
#[tokio::test]
async fn fail_missing_contract_call_data(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];

    let response_deoxys = deoxys.call(
        FunctionCall {
            contract_address: FieldElement::from_hex_be(STARKGATE_ETH_BRIDGE_ADDR).unwrap(),
            entry_point_selector: get_selector_from_name("balanceOf").unwrap(),
            calldata: vec![]
        },
        BlockId::Tag(BlockTag::Latest)
    ).await.err();

    assert_matches!(
        response_deoxys,
        Some(ProviderError::StarknetError(StarknetErrorWithMessage {
            message: _,
            code: MaybeUnknownErrorCode::Known(StarknetError::ContractError)
        }))
    );
}

/**
 * Unit test for `starknet_call`
 * 
 * purpose: function request `balanceOf` to StarkGate ETH bridge contract
 * fail case: invalid call data. This does not cause an error upon calling the contract but returns felt 0x0
 */
#[rstest]
#[tokio::test]
async fn fail_invalid_contract_call_data(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];

    let response_deoxys = deoxys.call(
        FunctionCall {
            contract_address: FieldElement::from_hex_be(STARKGATE_ETH_BRIDGE_ADDR).unwrap(),
            entry_point_selector: get_selector_from_name("balanceOf").unwrap(),
            calldata: vec![
                FieldElement::ZERO
            ]
        },
        BlockId::Tag(BlockTag::Latest)
    ).await.expect("Error waiting for response from Deoxys node");

    assert_eq!(response_deoxys, vec![FieldElement::ZERO, FieldElement::ZERO]);
}

/**
 * Unit test for `starknet_call`
 * 
 * purpose: function request `name` to StarkGate ETH bridge contract
 * fail case: too many arguments in call data
 */
#[rstest]
#[tokio::test]
async fn fail_too_many_call_data(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];

    let response_deoxys = deoxys.call(
        FunctionCall { 
            contract_address: FieldElement::from_hex_be(STARKGATE_ETH_BRIDGE_ADDR).unwrap(),
            entry_point_selector: get_selector_from_name("name").unwrap(), 
            calldata: vec![
                FieldElement::ZERO
            ]
        },
        BlockId::Tag(BlockTag::Latest)
    ).await.err();

    assert_matches!(
        response_deoxys,
        Some(ProviderError::StarknetError(StarknetErrorWithMessage {
            message: _,
            code: MaybeUnknownErrorCode::Known(StarknetError::ContractError)
        }))
    );
}

/**
 * Unit test for `starknet_call`
 * 
 * purpose: function request `name` to StarkGate ETH bridge contract
 * success case: should return 'Ether'
 */
#[rstest]
#[tokio::test]
async fn work_correct_call(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let response_deoxys = deoxys.call(
        FunctionCall { 
            contract_address: FieldElement::from_hex_be(STARKGATE_ETH_BRIDGE_ADDR).unwrap(),
            entry_point_selector: get_selector_from_name("name").unwrap(), 
            calldata: vec![]
        },
        BlockId::Tag(BlockTag::Latest)
    ).await.expect("Error waiting for response from Deoxys node");

    let response_pathfinder = pathfinder.call(
        FunctionCall { 
            contract_address: FieldElement::from_hex_be(STARKGATE_ETH_BRIDGE_ADDR).unwrap(), 
            entry_point_selector: get_selector_from_name("name").unwrap(), 
            calldata: vec![]
        },
        BlockId::Tag(BlockTag::Latest)
    ).await.expect("Error waiting for response from Pathfinder node");

    let response_expected = short_string!("Ether");

    assert_eq!(response_deoxys, vec![response_expected]);
    assert_eq!(response_deoxys, response_pathfinder);
}

/**
 * Unit test for `starknet_call`
 * 
 * purpose: function request `balanceOf` to StarkGate ETH bridge contract
 * success case: must return non-zero balance
 */
#[rstest]
#[tokio::test]
async fn work_correct_call_with_args(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let response_deoxys = deoxys.call(
        FunctionCall { 
            contract_address: FieldElement::from_hex_be(STARKGATE_ETH_BRIDGE_ADDR).unwrap(), 
            entry_point_selector: get_selector_from_name("balanceOf").unwrap(),
            calldata: vec![
                FieldElement::from_hex_be(CONTRACT_ADDR).unwrap()
            ]
        },
        BlockId::Tag(BlockTag::Latest)
    ).await.expect("Error waiting for response from Deoxys node");

    let response_pathfinder = pathfinder.call(
        FunctionCall { 
            contract_address: FieldElement::from_hex_be(STARKGATE_ETH_BRIDGE_ADDR).unwrap(), 
            entry_point_selector: get_selector_from_name("balanceOf").unwrap(),
            calldata: vec![
                FieldElement::from_hex_be(CONTRACT_ADDR).unwrap()
            ]
        },
        BlockId::Tag(BlockTag::Latest)
    ).await.expect("Error waiting for response from Pathfinder node");

    let balance_deoxys = u128::try_from(response_deoxys[0]).unwrap();

    assert!(balance_deoxys > 0);
    assert_eq!(response_deoxys, response_pathfinder);
}
