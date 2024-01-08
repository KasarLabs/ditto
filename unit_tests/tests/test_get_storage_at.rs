#![feature(assert_matches)]

mod common;
use common::*;

use std::assert_matches::assert_matches;
use std::collections::HashMap;

use starknet_providers::{JsonRpcClient, jsonrpc::HttpTransport, Provider, ProviderError, StarknetErrorWithMessage, MaybeUnknownErrorCode};
use starknet_core::types::{FieldElement, BlockId, BlockTag, StarknetError};

#[rstest]
#[tokio::test]
async fn fail_non_existing_block(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];

    let response_deoxys = deoxys.get_storage_at(
        FieldElement::from_hex_be(CONTRACT_ADDR).unwrap(),
        FieldElement::from_hex_be(CONTRACT_KEY).unwrap(),
        BlockId::Hash(FieldElement::ZERO)
    ).await.err();

    assert_matches!(
        response_deoxys,
        Some(ProviderError::StarknetError(StarknetErrorWithMessage {
            message: _,
            code: MaybeUnknownErrorCode::Known(StarknetError::BlockNotFound)
        }))
    )
}

#[rstest]
#[tokio::test]
async fn fail_non_existing_contract(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];

    let response_deoxys = deoxys.get_storage_at(
        FieldElement::ZERO,
        FieldElement::from_hex_be(CONTRACT_KEY).unwrap(),
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
async fn fail_invalid_storage_key(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];

    let response_deoxys = deoxys.get_storage_at(
        FieldElement::from_hex_be(CONTRACT_ADDR).unwrap(),
        FieldElement::from_hex_be("0x0").unwrap(),
        BlockId::Tag(BlockTag::Latest)
    ).await.expect("Error waiting for response from Deoxys client");

    assert_eq!(response_deoxys, FieldElement::ZERO);
}

#[rstest]
#[tokio::test]
async fn work_get_storage(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let response_deoxys = deoxys.get_storage_at(
        FieldElement::from_hex_be(CONTRACT_ADDR).unwrap(),
        FieldElement::from_hex_be(CONTRACT_KEY).unwrap(),
        BlockId::Tag(BlockTag::Latest)
    ).await.expect("Error waiting for response from Deoxys client");
    let response_pathfinder = pathfinder.get_storage_at(
        FieldElement::from_hex_be(CONTRACT_ADDR).unwrap(),
        FieldElement::from_hex_be(CONTRACT_KEY).unwrap(),
        BlockId::Tag(BlockTag::Latest)
    ).await.expect("Error waiting for response from Pathfinder client");

    assert_eq!(response_deoxys, response_pathfinder);
}