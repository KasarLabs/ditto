#![feature(assert_matches)]

mod common;
use common::*;
use starknet_core::types::MaybePendingBlockWithTxHashes;

use std::{assert_matches::assert_matches, collections::HashMap};

use starknet_core::types::{BlockId, BlockTag, FieldElement, StarknetError};
use starknet_providers::{
    jsonrpc::{HttpTransport, JsonRpcClient},
    Provider, ProviderError,
};
use unit_tests::constants::DEOXYS;

///
/// Unit test for `starknet_get_block_with_tx_hashes`
///
/// purpose: call getBlockWithTxHashes on invalid block.
/// fail case: invalid block.
///
#[require(spec_version = "0.5.1")]
#[rstest]
#[tokio::test]
async fn fail_non_existing_block(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];

    let response_deoxys = deoxys
        .get_block_with_tx_hashes(BlockId::Hash(FieldElement::ZERO))
        .await
        .err();

    assert_matches!(
        response_deoxys,
        Some(ProviderError::StarknetError(StarknetError::BlockNotFound))
    );
}

///
/// Unit test for `starknet_get_block_with_tx_hashes`
///
/// purpose: call getBlockWithTxHashes on latest validated block.
/// success case: retrieves valid block.
///
#[require(spec_version = "0.5.1")]
#[rstest]
#[tokio::test]
async fn work_existing_block(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let response_deoxys = deoxys
        .get_block_with_tx_hashes(BlockId::Tag(BlockTag::Latest))
        .await
        .expect("Error waiting for response from Deoxys node");
    let response_pathfinder = pathfinder
        .get_block_with_tx_hashes(BlockId::Tag(BlockTag::Latest))
        .await
        .expect("Error waiting for response from Deoxys node");

    let block_deoxys = match response_deoxys {
        MaybePendingBlockWithTxHashes::Block(block) => block,
        MaybePendingBlockWithTxHashes::PendingBlock(_) => {
            panic!("Expected block, got pending block")
        }
    };
    let block_pathfinder = match response_pathfinder {
        MaybePendingBlockWithTxHashes::Block(block) => block,
        MaybePendingBlockWithTxHashes::PendingBlock(_) => {
            panic!("Expected block, got pending block")
        }
    };

    assert_eq!(block_deoxys, block_pathfinder);
}

///
/// Unit test for `starknet_get_block_with_tx_hashes`
///
/// purpose: call getBlockWithTxHashes on pending block.
/// success case: retrieves valid pending block.
///
/// Note that this can fail at the last moments of a block being validated!!!
///
#[require(spec_version = "0.5.1")]
#[rstest]
#[tokio::test]
#[ignore = "Pending fails some times when called on the cusp of being accepted, need virtual sequencer"]
async fn work_pending_block(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let response_deoxys = deoxys
        .get_block_with_tx_hashes(BlockId::Tag(BlockTag::Pending))
        .await
        .expect("Error waiting for response from Deoxys node");

    let response_pathfinder = pathfinder
        .get_block_with_tx_hashes(BlockId::Tag(BlockTag::Pending))
        .await
        .expect("Error waiting for response from Deoxys node");

    let block_deoxys = match response_deoxys {
        MaybePendingBlockWithTxHashes::Block(_) => panic!("Expected pending block, got block"),
        MaybePendingBlockWithTxHashes::PendingBlock(block) => block,
    };
    let block_pathfinder = match response_pathfinder {
        MaybePendingBlockWithTxHashes::Block(_) => panic!("Expected pending block, got block"),
        MaybePendingBlockWithTxHashes::PendingBlock(block) => block,
    };

    assert_eq!(block_deoxys, block_pathfinder);
}
