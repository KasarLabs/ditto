#![feature(assert_matches)]

use rpc_test::test_config::TestConfig;
use starknet::{providers::{
    jsonrpc::{HttpTransport, JsonRpcClient},
    Provider, ProviderError, StarknetErrorWithMessage, MaybeUnknownErrorCode
}, core::types::{BlockId, FieldElement, StarknetError, BlockTag, BroadcastedTransaction, BroadcastedInvokeTransaction}};
use starknet::core::utils::get_selector_from_name;
use url::Url;
use std::assert_matches::assert_matches;
//use starknet_providers::ProviderError::StarknetError as StarknetProviderError;

const ACCOUNT_CONTRACT: &str = "";
const TEST_CONTRACT_ADDRESS: &str = "";


#[tokio::test]
async fn fail_non_existing_block() {
    let config = TestConfig::new("./secret.json").unwrap();
    let deoxys = JsonRpcClient::new(HttpTransport::new(
        Url::parse(&config.deoxys).unwrap()
    ));

    let ok_invoke_transaction = BroadcastedTransaction::Invoke(BroadcastedInvokeTransaction {
        max_fee: FieldElement::ZERO,
        signature: vec![],
        nonce: FieldElement::ZERO,
        sender_address: FieldElement::from_hex_be(ACCOUNT_CONTRACT).unwrap(),
        calldata: vec![
            FieldElement::from_hex_be(TEST_CONTRACT_ADDRESS).unwrap(),
            get_selector_from_name("sqrt").unwrap(),
            FieldElement::from_hex_be("1").unwrap(),
            FieldElement::from(81u8),
        ],
        is_query: true,
    });

    assert_matches!(
        deoxys.estimate_fee(&vec![ok_invoke_transaction], BlockId::Hash(FieldElement::ZERO)).await,
        Err(ProviderError::StarknetError(StarknetErrorWithMessage { code: MaybeUnknownErrorCode::Known(code), .. })) if code == StarknetError::BlockNotFound
    );

}

#[tokio::test]
async fn fail_if_one_txn_cannot_be_executed() {
    let config = TestConfig::new("./secret.json").unwrap();
    let deoxys = JsonRpcClient::new(HttpTransport::new(
        Url::parse(&config.deoxys).unwrap()
    ));

    let alchemy = JsonRpcClient::new(HttpTransport::new(
        Url::parse(&config.alchemy).unwrap()
    ));

    let bad_invoke_transaction = BroadcastedTransaction::Invoke(BroadcastedInvokeTransaction {
        max_fee: FieldElement::default(),
        nonce: FieldElement::ZERO,
        sender_address: FieldElement::default(),
        signature: vec![],
        calldata: vec![FieldElement::from_hex_be("0x0").unwrap()],
        is_query: true,
    });

    let ok_invoke_transaction = BroadcastedTransaction::Invoke(BroadcastedInvokeTransaction {
        max_fee: FieldElement::ZERO,
        signature: vec![],
        nonce: FieldElement::ZERO,
        sender_address: FieldElement::from_hex_be(ACCOUNT_CONTRACT).unwrap(),
        calldata: vec![
            FieldElement::from_hex_be(TEST_CONTRACT_ADDRESS).unwrap(),
            get_selector_from_name("sqrt").unwrap(),
            FieldElement::from_hex_be("1").unwrap(),
            FieldElement::from(81u8),
        ],
        is_query: true,
    });

    let alchemy_bad_invoke_transaction = BroadcastedTransaction::Invoke(BroadcastedInvokeTransaction {
        max_fee: FieldElement::default(),
        nonce: FieldElement::ZERO,
        sender_address: FieldElement::default(),
        signature: vec![],
        calldata: vec![FieldElement::from_hex_be("0x0").unwrap()],
        is_query: true,
    });

    let alchemy_ok_invoke_transaction = BroadcastedTransaction::Invoke(BroadcastedInvokeTransaction {
        max_fee: FieldElement::ZERO,
        signature: vec![],
        nonce: FieldElement::ZERO,
        sender_address: FieldElement::from_hex_be(ACCOUNT_CONTRACT).unwrap(),
        calldata: vec![
            FieldElement::from_hex_be(TEST_CONTRACT_ADDRESS).unwrap(),
            get_selector_from_name("sqrt").unwrap(),
            FieldElement::from_hex_be("1").unwrap(),
            FieldElement::from(81u8),
        ],
        is_query: true,
    });

    // assert_matches!(
    //     deoxys.estimate_fee(&vec![
    //         bad_invoke_transaction,
    //         ok_invoke_transaction,
    //     ], BlockId::Tag(BlockTag::Latest)).await,
    //     Err(ProviderError::StarknetError(StarknetErrorWithMessage { code: MaybeUnknownErrorCode::Known(code), .. })) if code == StarknetError::ContractError
    // );

    let deoxys_ok_result = deoxys.estimate_fee(&vec![ok_invoke_transaction], BlockId::Tag(BlockTag::Latest)).await;
    let deoxys_bad_result = deoxys.estimate_fee(&vec![bad_invoke_transaction], BlockId::Tag(BlockTag::Latest)).await;
    
    let alchemy_ok_result = alchemy.estimate_fee(&vec![alchemy_ok_invoke_transaction], BlockId::Tag(BlockTag::Latest)).await;
    let alchemy_bad_result = alchemy.estimate_fee(&vec![alchemy_bad_invoke_transaction], BlockId::Tag(BlockTag::Latest)).await;
    
    assert_matches!(
        (deoxys_ok_result, alchemy_ok_result),
        (Ok(_), Ok(_))
    );

    assert_matches!(
        (deoxys_bad_result, alchemy_bad_result),
        (Ok(_), Ok(_))
    );

}

#[tokio::test]
async fn works_ok(){
    let config = TestConfig::new("./secret.json").unwrap();
    let deoxys = JsonRpcClient::new(HttpTransport::new(
        Url::parse(&config.deoxys).unwrap()
    ));

    let alchemy = JsonRpcClient::new(HttpTransport::new(
        Url::parse(&config.alchemy).unwrap()
    ));

    let tx = BroadcastedInvokeTransaction {
        max_fee: FieldElement::ZERO,
        signature: vec![],
        nonce: FieldElement::ZERO,
        sender_address: FieldElement::from_hex_be(ACCOUNT_CONTRACT).unwrap(),
        calldata: vec![
            FieldElement::from_hex_be(TEST_CONTRACT_ADDRESS).unwrap(),
            get_selector_from_name("sqrt").unwrap(),
            FieldElement::from_hex_be("1").unwrap(),
            FieldElement::from(81u8),
        ],
        is_query: true,
    };

    let tx_alchemy = BroadcastedInvokeTransaction {
        max_fee: FieldElement::ZERO,
        signature: vec![],
        nonce: FieldElement::ZERO,
        sender_address: FieldElement::from_hex_be(ACCOUNT_CONTRACT).unwrap(),
        calldata: vec![
            FieldElement::from_hex_be(TEST_CONTRACT_ADDRESS).unwrap(),
            get_selector_from_name("sqrt").unwrap(),
            FieldElement::from_hex_be("1").unwrap(),
            FieldElement::from(81u8),
        ],
        is_query: true,
    };

    let invoke_transaction_deoxys = BroadcastedTransaction::Invoke(tx.clone());
    let invoke_transaction_alchemy = BroadcastedTransaction::Invoke(tx_alchemy.clone());

    let invoke_transaction_2 =
        BroadcastedTransaction::Invoke(BroadcastedInvokeTransaction { nonce: FieldElement::ONE, ..tx });

    let invoke_transaction_3 =
        BroadcastedTransaction::Invoke(BroadcastedInvokeTransaction { nonce: FieldElement::ONE, ..tx_alchemy });

    let deoxys_estimates =
        deoxys.estimate_fee(&vec![invoke_transaction_deoxys, invoke_transaction_2], BlockId::Tag(BlockTag::Latest)).await.unwrap();
    
    let alchemy_estimates =
        alchemy.estimate_fee(&vec![invoke_transaction_alchemy, invoke_transaction_3], BlockId::Tag(BlockTag::Latest)).await.unwrap();

    // TODO: instead execute the tx and check that the actual fee are the same as the estimated ones
    assert_eq!(deoxys_estimates.len(), 2);
    assert_eq!(deoxys_estimates[0].overall_fee, 420);
    assert_eq!(deoxys_estimates[1].overall_fee, 420);
    
    assert_eq!(deoxys_estimates[0].gas_consumed, 0);
    assert_eq!(deoxys_estimates[1].gas_consumed, 0);

    assert_eq!(deoxys_estimates.len(), alchemy_estimates.len());
    assert_eq!(deoxys_estimates[0].overall_fee, alchemy_estimates[0].overall_fee);
    assert_eq!(deoxys_estimates[1].overall_fee, alchemy_estimates[1].overall_fee);
    
    assert_eq!(deoxys_estimates[0].gas_consumed, alchemy_estimates[0].gas_consumed);
    assert_eq!(deoxys_estimates[1].gas_consumed, alchemy_estimates[1].gas_consumed);

    //Ok(())
}


