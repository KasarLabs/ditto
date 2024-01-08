#![feature(assert_matches)]

mod common;
use common::*;
use starknet_core::types::MaybePendingBlockWithTxHashes;

use std::{assert_matches::assert_matches, collections::HashMap};

use starknet_providers::{jsonrpc::{HttpTransport, JsonRpcClient}, Provider, ProviderError, StarknetErrorWithMessage, MaybeUnknownErrorCode};
use starknet_core::types::{BlockId, FieldElement, StarknetError, BlockTag};
use unit_tests::constants::DEOXYS;

#[rstest]
#[tokio::test]
async fn fail_non_existing_block(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];

    let response_deoxys = deoxys.get_block_with_tx_hashes(BlockId::Hash(FieldElement::ZERO)).await.err();

    assert_matches!(
        response_deoxys, 
        Some(ProviderError::StarknetError(StarknetErrorWithMessage {
            message: _,
            code: MaybeUnknownErrorCode::Known(StarknetError::BlockNotFound)
        }
    )));
}

#[rstest]
#[tokio::test]
async fn work_existing_block(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let response_deoxys = deoxys.get_block_with_tx_hashes(BlockId::Tag(BlockTag::Latest)).await
        .expect("Error waiting for response from Deoxys node");
    let response_pathfinder = pathfinder.get_block_with_tx_hashes(BlockId::Tag(BlockTag::Latest)).await
        .expect("Error waiting for response from Deoxys node");

    let block_deoxys = match response_deoxys {
        MaybePendingBlockWithTxHashes::Block(block) => block,
        MaybePendingBlockWithTxHashes::PendingBlock(_) => panic!("Expected block, got pending block"),
    };
    let block_pathfinder = match response_pathfinder {
        MaybePendingBlockWithTxHashes::Block(block) => block,
        MaybePendingBlockWithTxHashes::PendingBlock(_) => panic!("Expected block, got pending block"),
    };

    assert_eq!(block_deoxys, block_pathfinder);
}