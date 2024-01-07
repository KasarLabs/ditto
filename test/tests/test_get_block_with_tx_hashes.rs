#![feature(assert_matches)]

use rpc_test::test_config::TestConfig;
use starknet::{providers::{
    jsonrpc::{HttpTransport, JsonRpcClient},
    Provider, ProviderError, StarknetErrorWithMessage, MaybeUnknownErrorCode
}, core::types::{BlockId, FieldElement, StarknetError, BlockTag}};
use url::Url;
use std::assert_matches::assert_matches;

#[tokio::test]
async fn fail_non_existing_block() {
    let config = TestConfig::new("./secret.json").unwrap();
    let deoxys = JsonRpcClient::new(HttpTransport::new(
        Url::parse(&config.deoxys).unwrap()
    ));

    let response_deoxys = deoxys.get_block_with_tx_hashes(BlockId::Hash(FieldElement::ZERO)).await.err();

    assert_matches!(
        response_deoxys, 
        Some(ProviderError::StarknetError(StarknetErrorWithMessage {
            message: _,
            code: MaybeUnknownErrorCode::Known(StarknetError::BlockNotFound)
        }
    )));
}

#[tokio::test]
async fn work_existing_block() {
    let config = TestConfig::new("./secret.json").expect("'./secret.json' must contain correct node urls");
    let deoxys = JsonRpcClient::new(HttpTransport::new(
        Url::parse(&config.deoxys).unwrap()
    ));
    let alchemy = JsonRpcClient::new(HttpTransport::new(
        Url::parse(&config.alchemy).unwrap()
    ));

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