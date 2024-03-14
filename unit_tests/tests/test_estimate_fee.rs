#![feature(assert_matches)]

mod common;
use common::*;

use starknet_core::types::{BlockId, BlockTag, FieldElement, StarknetError};
use starknet_providers::{
    jsonrpc::{HttpTransport, JsonRpcClient, JsonRpcError},
    Provider
};
use std::collections::HashMap;
use unit_tests::{BadTransactionFactory, OkTransactionFactory, TransactionFactory};

#[rstest]
#[tokio::test]
#[ignore = "Fix failing unwrap due to empty constant"]
async fn fail_non_existing_block(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];

    let ok_invoke_transaction = OkTransactionFactory::build(Some(FieldElement::ZERO));

    let response_deoxys = deoxys
        .estimate_fee(
            &vec![ok_invoke_transaction],
            BlockId::Hash(FieldElement::ZERO),
        )
        .await;

    assert!(
        response_deoxys.is_ok(),
        "Expected an error, but got a result"
    );

    if let Err(error) = response_deoxys {
        let is_correct_error = checking_error_format(
            &error,
            StarknetError::InvalidTransactionHash,
        );

        assert!(
            is_correct_error,
            "Expected InvalidTransactionHash error, but got a different error"
        );
    }
}

#[rstest]
#[tokio::test]
#[ignore = "Fix failing unwrap due to empty constant"]
async fn fail_if_one_txn_cannot_be_executed(
    clients: HashMap<String, JsonRpcClient<HttpTransport>>,
) {
    let deoxys = &clients[PATHFINDER];

    let bad_invoke_transaction = BadTransactionFactory::build(None);

    let response_deoxys = deoxys
        .estimate_fee(
            vec![bad_invoke_transaction.clone()],
            BlockId::Tag(BlockTag::Latest),
        )
        .await;

    let expected_error = JsonRpcError {
        code: -32602,
        message: "Invalid params".to_string(),
        data: None,
    };
    
    assert!(
        response_deoxys.is_err(),
        "Expected an error response, but got Ok. Expected error: {:?}",
        expected_error
    );
}

#[rstest]
#[tokio::test]
#[ignore = "Fix failing unwrap due to empty constant"]
async fn works_ok(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let ok_deoxys_invoke = OkTransactionFactory::build(Some(FieldElement::ZERO));
    let ok_deoxys_invoke_1 = OkTransactionFactory::build(Some(FieldElement::ONE));
    let ok_deoxys_invoke_2 = OkTransactionFactory::build(Some(FieldElement::TWO));

    let ok_pathfinder_invoke = OkTransactionFactory::build(Some(FieldElement::ZERO));
    let ok_pathfinder_invoke_1 = OkTransactionFactory::build(Some(FieldElement::ONE));
    let ok_pathfinder_invoke_2 = OkTransactionFactory::build(Some(FieldElement::TWO));

    let deoxys_estimates = deoxys
        .estimate_fee(
            &vec![ok_deoxys_invoke, ok_deoxys_invoke_1, ok_deoxys_invoke_2],
            BlockId::Tag(BlockTag::Latest),
        )
        .await
        .unwrap();

    let pathfinder_estimates = pathfinder
        .estimate_fee(
            &vec![
                ok_pathfinder_invoke,
                ok_pathfinder_invoke_1,
                ok_pathfinder_invoke_2,
            ],
            BlockId::Tag(BlockTag::Latest),
        )
        .await
        .unwrap();

    assert_eq!(deoxys_estimates, pathfinder_estimates);
}
