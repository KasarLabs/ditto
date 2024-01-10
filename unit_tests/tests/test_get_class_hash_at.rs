#![feature(assert_matches)]

mod common;
use common::*;

use std::{assert_matches::assert_matches, collections::HashMap};

use starknet_providers::{JsonRpcClient, jsonrpc::HttpTransport, Provider, ProviderError, StarknetErrorWithMessage, MaybeUnknownErrorCode};
use starknet_core::types::{BlockId, FieldElement, StarknetError, BlockTag};

///
/// Unit test for `starknet_getClassHashAt`
/// 
/// purpose: call getClassHashAt on invalid block.
/// fail case: invalid block hash.
/// 
#[rstest]
#[tokio::test]
async fn fail_non_existing_block(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];

    let response_deoxys = deoxys.get_class_hash_at(
        BlockId::Hash(FieldElement::ZERO),
        FieldElement::from_hex_be(STARKGATE_ETH_CONTRACT_ADDR).unwrap()
    ).await.err();

    assert_matches!(
        response_deoxys,
        Some(ProviderError::StarknetError(StarknetErrorWithMessage {
            message: _,
            code: MaybeUnknownErrorCode::Known(StarknetError::BlockNotFound)
        }))
    )
}

///
/// Unit test for `starknet_getClassHashAt`
/// 
/// purpose: call getClassHashAt on non-existent contract.
/// fail case: invalid contract hash.
/// 
#[rstest]
#[tokio::test]
async fn fail_non_existing_contract(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];

    let response_deoxys = deoxys.get_class_hash_at(
        BlockId::Tag(BlockTag::Latest),
        FieldElement::from_hex_be(INVALID_CONTRACT_ADDR).unwrap()
    ).await.err();

    assert_matches!(
        response_deoxys,
        Some(ProviderError::StarknetError(StarknetErrorWithMessage {
            message: _,
            code: MaybeUnknownErrorCode::Known(StarknetError::ContractNotFound)
        }))
    )
}

///
/// Unit test for `starknet_getClassHashAt`
/// 
/// purpose: call getClassHashAt on latest block.
/// success case: retrieve valid class hash.
/// 
#[rstest]
#[tokio::test]
async fn work_block_latest(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let class_hash_deoxys = deoxys.get_class_hash_at(
        BlockId::Tag(BlockTag::Latest),
        FieldElement::from_hex_be(STARKGATE_ETH_CONTRACT_ADDR).unwrap()
    ).await.expect("Error waiting for response from Deoxys node");
    let class_hash_pathfinder = pathfinder.get_class_hash_at(
        BlockId::Tag(BlockTag::Latest),
        FieldElement::from_hex_be(STARKGATE_ETH_CONTRACT_ADDR).unwrap()
    ).await.expect("Error waiting for response from Pathfinder node");

    assert_eq!(class_hash_deoxys, class_hash_pathfinder);
}

///
/// Unit test for `starknet_getClassHashAt`
/// 
/// purpose: call getClassHashAt on pending block.
/// success case: retrieve valid class hash.
/// 
#[rstest]
#[tokio::test]
async fn work_block_pending(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let class_hash_deoxys = deoxys.get_class_hash_at(
        BlockId::Tag(BlockTag::Pending),
        FieldElement::from_hex_be(STARKGATE_ETH_CONTRACT_ADDR).unwrap()
    ).await.expect("Error waiting for response from Deoxys node");
    let class_hash_pathfinder = pathfinder.get_class_hash_at(
        BlockId::Tag(BlockTag::Pending),
        FieldElement::from_hex_be(STARKGATE_ETH_CONTRACT_ADDR).unwrap()
    ).await.expect("Error waiting for response from Pathfinder node");

    assert_eq!(class_hash_deoxys, class_hash_pathfinder);
}