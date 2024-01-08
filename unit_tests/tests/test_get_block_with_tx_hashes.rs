#![feature(assert_matches)]

mod common;
use common::*;

use std::{assert_matches::assert_matches, collections::HashMap};

use starknet::{providers::{jsonrpc::{HttpTransport, JsonRpcClient}, Provider, ProviderError, StarknetErrorWithMessage, MaybeUnknownErrorCode}, core::types::{BlockId, FieldElement, StarknetError, BlockTag}};
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
    let alchemy = &clients[ALCHEMY];

    let response_deoxys = deoxys.get_block_with_tx_hashes(BlockId::Tag(BlockTag::Latest)).await
        .expect("Error waiting for response from Deoxys node");
    let response_alchemy = alchemy.get_block_with_tx_hashes(BlockId::Tag(BlockTag::Latest)).await
        .expect("Error waiting for response from Deoxys node");

    let block_deoxys = match response_deoxys {
        starknet::core::types::MaybePendingBlockWithTxHashes::Block(block) => block,
        starknet::core::types::MaybePendingBlockWithTxHashes::PendingBlock(_) => panic!("Expected block, got pending block"),
    };
    let block_alchemy = match response_alchemy {
        starknet::core::types::MaybePendingBlockWithTxHashes::Block(block) => block,
        starknet::core::types::MaybePendingBlockWithTxHashes::PendingBlock(_) => panic!("Expected block, got pending block"),
    };

    assert_eq!(block_deoxys, block_alchemy);
}