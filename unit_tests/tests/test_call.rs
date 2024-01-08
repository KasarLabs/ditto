#![feature(assert_matches)]

mod common;
use std::{collections::HashMap, assert_matches::assert_matches};

use common::*;
use starknet_core::{types::{FunctionCall, BlockTag, BlockId, FieldElement, StarknetError}, utils::get_selector_from_name};
use starknet_providers::{JsonRpcClient, jsonrpc::HttpTransport, Provider, ProviderError, StarknetErrorWithMessage, MaybeUnknownErrorCode};

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

    // hash of string value 'Ether'
    let response_expected = FieldElement::from_hex_be("0x4574686572").unwrap();

    assert_eq!(response_deoxys[0], response_expected);
    assert_eq!(response_deoxys, response_pathfinder);
}

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

    let balance = u128::try_from(response_deoxys[0]).unwrap();

    println!("balance: {balance}");

    assert!(balance > 0);
    assert_eq!(response_deoxys, response_pathfinder);
}