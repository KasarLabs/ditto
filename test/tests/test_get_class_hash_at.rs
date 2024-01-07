#![feature(assert_matches)]

const STARKGATE_ETH_CONTRACT_ADDR: &str = "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7";
const INVALID_CONTRACT_ADDR: &str = "0x4269DEADBEEF";

use std::assert_matches::assert_matches;

use rpc_test::test_config::TestConfig;
use starknet::{providers::{JsonRpcClient, jsonrpc::HttpTransport, Provider, ProviderError, StarknetErrorWithMessage, MaybeUnknownErrorCode}, core::types::{BlockId, FieldElement, StarknetError, BlockTag}};
use url::Url;

#[tokio::test]
async fn fail_non_existing_block() {
    let config = TestConfig::new("./secret.json").expect("Error loading test config");
    let deoxys = JsonRpcClient::new(HttpTransport::new(
        Url::parse(&config.deoxys).expect("Error parsing Deoxys api url")
    ));

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

#[tokio::test]
async fn fail_non_existing_contract() {
    let config = TestConfig::new("./secret.json").expect("Error loading test config");
    let deoxys = JsonRpcClient::new(HttpTransport::new(
        Url::parse(&config.deoxys).expect("Error parsing Deoxys api rul")
    ));

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

#[tokio::test]
async fn work_existing_block_and_contract() {
    let config = TestConfig::new("./secret.json").expect("Error loading test config");
    let deoxys = JsonRpcClient::new(HttpTransport::new(
        Url::parse(&config.deoxys).expect("Error parsing Deoxys api rul")
    ));

    let class_hash_deoxys = deoxys.get_class_hash_at(
        BlockId::Tag(BlockTag::Latest),
        FieldElement::from_hex_be(STARKGATE_ETH_CONTRACT_ADDR).unwrap()
    ).await.expect("Error waiting for response from Deoxys node");
    let class_hash_alchemy = deoxys.get_class_hash_at(
        BlockId::Tag(BlockTag::Latest),
        FieldElement::from_hex_be(STARKGATE_ETH_CONTRACT_ADDR).unwrap()
    ).await.expect("Error waiting for response from Alchemy node");

    assert_eq!(class_hash_deoxys, class_hash_alchemy);
}