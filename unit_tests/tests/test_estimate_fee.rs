#![feature(assert_matches)]

mod common;
use common::*;

use starknet_core::types::{BlockId, BlockTag, FieldElement, StarknetError};
use starknet_providers::{jsonrpc::{HttpTransport, JsonRpcClient}, MaybeUnknownErrorCode, Provider, ProviderError, StarknetErrorWithMessage,};
use unit_tests::{OkTransactionFactory, TransactionFactory, BadTransactionFactory};
use std::assert_matches::assert_matches;
use std::collections::HashMap;

#[rstest]
#[tokio::test]
async fn fail_non_existing_block(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];

    let ok_invoke_transaction = OkTransactionFactory::new(Some(FieldElement::ZERO));

    assert_matches!(
        deoxys.estimate_fee(&vec![ok_invoke_transaction], BlockId::Hash(FieldElement::ZERO)).await,
        Err(ProviderError::StarknetError(StarknetErrorWithMessage { code: MaybeUnknownErrorCode::Known(code), .. })) if code == StarknetError::BlockNotFound
    );
}

#[rstest]
#[tokio::test]
async fn fail_if_one_txn_cannot_be_executed(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let bad_invoke_transaction = BadTransactionFactory::new(None);

    let result_deoxys = deoxys
        .estimate_fee(
            vec![bad_invoke_transaction.clone()],
            BlockId::Tag(BlockTag::Latest),
        )
        .await
        .unwrap();

    // FIX: this causes an error during the tests
    let result_pathfinder = pathfinder
        .estimate_fee(vec![bad_invoke_transaction], BlockId::Tag(BlockTag::Latest))
        .await
        .unwrap();

    assert_eq!(result_deoxys, result_pathfinder);
}

#[rstest]
#[tokio::test]
async fn works_ok(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let ok_deoxys_invoke = OkTransactionFactory::new(Some(FieldElement::ZERO));
    let ok_deoxys_invoke_1 = OkTransactionFactory::new(Some(FieldElement::ONE));
    let ok_deoxys_invoke_2 = OkTransactionFactory::new(Some(FieldElement::TWO));

    let ok_pathfinder_invoke = OkTransactionFactory::new(Some(FieldElement::ZERO));
    let ok_pathfinder_invoke_1 = OkTransactionFactory::new(Some(FieldElement::ONE));
    let ok_pathfinder_invoke_2 = OkTransactionFactory::new(Some(FieldElement::TWO));

    let deoxys_estimates = deoxys
        .estimate_fee(
            &vec![ok_deoxys_invoke, ok_deoxys_invoke_1, ok_deoxys_invoke_2],
            BlockId::Tag(BlockTag::Latest),
        )
        .await
        .unwrap();

    let pathfinder_estimates = pathfinder
        .estimate_fee(
            &vec![ok_pathfinder_invoke, ok_pathfinder_invoke_1, ok_pathfinder_invoke_2],
            BlockId::Tag(BlockTag::Latest),
        )
        .await
        .unwrap();

    assert_eq!(deoxys_estimates, pathfinder_estimates)
}