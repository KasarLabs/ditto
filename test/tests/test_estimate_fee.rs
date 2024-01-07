#![feature(assert_matches)]

use rpc_test::test_config::TestConfig;
use starknet_core::types::{
    BlockId, BlockTag, BroadcastedInvokeTransaction, BroadcastedTransaction, FieldElement,
    StarknetError,
};
use starknet_core::utils::get_selector_from_name;
use starknet_providers::{
    jsonrpc::{HttpTransport, JsonRpcClient},
    MaybeUnknownErrorCode, Provider, ProviderError, StarknetErrorWithMessage,
};
use std::assert_matches::assert_matches;
use url::Url;

const ACCOUNT_CONTRACT: &str = "";
const TEST_CONTRACT_ADDRESS: &str = "";

fn load_ok_invoke_transaction(nonce: FieldElement) -> BroadcastedTransaction {
    BroadcastedTransaction::Invoke(BroadcastedInvokeTransaction {
        max_fee: FieldElement::ZERO,
        signature: vec![],
        nonce: nonce,
        sender_address: FieldElement::from_hex_be(ACCOUNT_CONTRACT).unwrap(),
        calldata: vec![
            FieldElement::from_hex_be(TEST_CONTRACT_ADDRESS).unwrap(),
            get_selector_from_name("sqrt").unwrap(),
            FieldElement::from_hex_be("1").unwrap(),
            FieldElement::from(81u8),
        ],
        is_query: true,
    })
}

fn load_bad_invoke_transaction() -> BroadcastedTransaction {
    BroadcastedTransaction::Invoke(BroadcastedInvokeTransaction {
        max_fee: FieldElement::default(),
        nonce: FieldElement::ZERO,
        sender_address: FieldElement::default(),
        signature: vec![],
        calldata: vec![FieldElement::from_hex_be("0x0").unwrap()],
        is_query: true,
    })
}

#[tokio::test]
async fn fail_non_existing_block() {
    let config = TestConfig::new("./secret.json").unwrap();
    let deoxys = JsonRpcClient::new(HttpTransport::new(Url::parse(&config.deoxys).unwrap()));

    let ok_invoke_transaction = load_ok_invoke_transaction(FieldElement::ZERO);

    assert_matches!(
        deoxys.estimate_fee(&vec![ok_invoke_transaction], BlockId::Hash(FieldElement::ZERO)).await,
        Err(ProviderError::StarknetError(StarknetErrorWithMessage { code: MaybeUnknownErrorCode::Known(code), .. })) if code == StarknetError::BlockNotFound
    );
}

#[tokio::test]
async fn fail_if_one_txn_cannot_be_executed() {
    let config = TestConfig::new("./secret.json").unwrap();
    let deoxys = JsonRpcClient::new(HttpTransport::new(Url::parse(&config.deoxys).unwrap()));
    let alchemy = JsonRpcClient::new(HttpTransport::new(Url::parse(&config.alchemy).unwrap()));

    let bad_invoke_transaction = load_bad_invoke_transaction();

    let result_deoxys = deoxys
        .estimate_fee(
            vec![bad_invoke_transaction.clone()],
            BlockId::Tag(BlockTag::Latest),
        )
        .await
        .unwrap();

    let result_alchemy = alchemy
        .estimate_fee(vec![bad_invoke_transaction], BlockId::Tag(BlockTag::Latest))
        .await
        .unwrap();

    assert_eq!(result_deoxys, result_alchemy);
}

#[tokio::test]
async fn works_ok() {
    let config = TestConfig::new("./secret.json").unwrap();
    let deoxys = JsonRpcClient::new(HttpTransport::new(Url::parse(&config.deoxys).unwrap()));
    let alchemy = JsonRpcClient::new(HttpTransport::new(Url::parse(&config.alchemy).unwrap()));

    let ok_deoxys_invoke = load_ok_invoke_transaction(FieldElement::ZERO);
    let ok_deoxys_invoke_1 = load_ok_invoke_transaction(FieldElement::ONE);
    let ok_deoxys_invoke_2 = load_ok_invoke_transaction(FieldElement::TWO);

    let ok_alchemy_invoke = load_ok_invoke_transaction(FieldElement::ZERO);
    let ok_alchemy_invoke_1 = load_ok_invoke_transaction(FieldElement::ONE);
    let ok_alchemy_invoke_2 = load_ok_invoke_transaction(FieldElement::TWO);

    let deoxys_estimates = deoxys
        .estimate_fee(
            &vec![ok_deoxys_invoke, ok_deoxys_invoke_1, ok_deoxys_invoke_2],
            BlockId::Tag(BlockTag::Latest),
        )
        .await
        .unwrap();

    let alchemy_estimates = alchemy
        .estimate_fee(
            &vec![ok_alchemy_invoke, ok_alchemy_invoke_1, ok_alchemy_invoke_2],
            BlockId::Tag(BlockTag::Latest),
        )
        .await
        .unwrap();

    assert_eq!(deoxys_estimates, alchemy_estimates)
}
