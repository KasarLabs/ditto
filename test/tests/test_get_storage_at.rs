#![feature(assert_matches)]

const CONTRACT_ADDR: &str = "0x03a20d4f7b4229e7c4863dab158b4d076d7f454b893d90a62011882dc4caca2a";
const CONTRACT_KEY: &str = "0x00f920571b9f85bdd92a867cfdc73319d0f8836f0e69e06e4c5566b6203f75cc";

use std::assert_matches::assert_matches;

use rpc_test::test_config::TestConfig;
use starknet::{providers::{JsonRpcClient, jsonrpc::HttpTransport, Provider, ProviderError, StarknetErrorWithMessage, MaybeUnknownErrorCode}, core::types::{FieldElement, BlockId, BlockTag, StarknetError}};
use url::Url;

#[tokio::test]
async fn fail_non_existing_block() {
    let config = TestConfig::new("./secret.json").expect("Error loading tests config");
    let deoxys = JsonRpcClient::new(HttpTransport::new(
        Url::parse(&config.deoxys).expect("Error parsing Deoxys api url")
    ));

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

#[tokio::test]
async fn fail_non_existing_contract() {
    let config = TestConfig::new("./secret.json").expect("Error loading tests config");
    let deoxys = JsonRpcClient::new(HttpTransport::new(
        Url::parse(&config.deoxys).expect("Error parsing Deoxys api url")
    ));

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

#[tokio::test]
async fn fail_invalid_storage_key() {
    let config = TestConfig::new("./secret.json").expect("Error loading tests config");
    let deoxys = JsonRpcClient::new(HttpTransport::new(
        Url::parse(&config.deoxys).expect("Error parsing Deoxys api url")
    ));

    let response_deoxys = deoxys.get_storage_at(
        FieldElement::from_hex_be(CONTRACT_ADDR).unwrap(),
        FieldElement::from_hex_be("0x0").unwrap(),
        BlockId::Tag(BlockTag::Latest)
    ).await.expect("Error waiting for response from Deoxys client");

    assert_eq!(response_deoxys, FieldElement::ZERO);
}

#[tokio::test]
async fn work_get_storage() {
    let config = TestConfig::new("./secret.json").expect("Error loading tests config");
    let deoxys = JsonRpcClient::new(HttpTransport::new(
        Url::parse(&config.deoxys).expect("Error parsing Deoxys api url")
    ));
    let alchemy = JsonRpcClient::new(HttpTransport::new(
        Url::parse(&config.deoxys).expect("Error parsing Alchemy api url")
    ));

    let response_deoxys = deoxys.get_storage_at(
        FieldElement::from_hex_be(CONTRACT_ADDR).unwrap(),
        FieldElement::from_hex_be(CONTRACT_KEY).unwrap(),
        BlockId::Tag(BlockTag::Latest)
    ).await.expect("Error waiting for response from Deoxys client");
    let response_alchemy = alchemy.get_storage_at(
        FieldElement::from_hex_be(CONTRACT_ADDR).unwrap(),
        FieldElement::from_hex_be(CONTRACT_KEY).unwrap(),
        BlockId::Tag(BlockTag::Latest)
    ).await.expect("Error waiting for response from Alchemy client");

    assert_eq!(response_deoxys, response_alchemy);
}