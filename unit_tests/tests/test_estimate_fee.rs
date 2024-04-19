#![feature(assert_matches)]

mod common;
use common::*;

use starknet_core::types::{
    BlockId, BlockTag, FieldElement, SimulationFlagForEstimateFee, StarknetError,
};
use starknet_providers::{
    jsonrpc::{HttpTransport, JsonRpcClient, JsonRpcError},
    Provider,
};
use std::collections::HashMap;
use unit_tests::{BadTransactionFactory, OkTransactionFactory, TransactionFactory};

//TODO(Tbelleng : Add Simulation Flag to params)
#[rstest]
#[tokio::test]
#[ignore = "Fix failing unwrap due to empty constant"]
async fn fail_non_existing_block(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];

    let ok_invoke_transaction = OkTransactionFactory::build(Some(FieldElement::ZERO));
    let simulation_flag = vec![SimulationFlagForEstimateFee::SkipValidate];

    let response_deoxys = deoxys
        .estimate_fee(
            &vec![ok_invoke_transaction],
            simulation_flag,
            BlockId::Hash(FieldElement::ZERO),
        )
        .await;

    assert!(
        response_deoxys.is_ok(),
        "Expected an error, but got a result"
    );

    if let Err(error) = response_deoxys {
        let is_correct_error = checking_error_format(&error, StarknetError::BlockNotFound);

        assert!(
            is_correct_error,
            "Expected BlockNotFound error, but got a different error"
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
    let simulate_flag = vec![SimulationFlagForEstimateFee::SkipValidate];

    let response_deoxys = deoxys
        .estimate_fee(
            vec![bad_invoke_transaction.clone()],
            simulate_flag,
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

    let block_number = get_block_setting();

    let ok_deoxys_invoke = OkTransactionFactory::build(Some(FieldElement::ZERO));
    let ok_deoxys_invoke_1 = OkTransactionFactory::build(Some(FieldElement::ONE));
    let ok_deoxys_invoke_2 = OkTransactionFactory::build(Some(FieldElement::TWO));

    let ok_pathfinder_invoke = OkTransactionFactory::build(Some(FieldElement::ZERO));
    let ok_pathfinder_invoke_1 = OkTransactionFactory::build(Some(FieldElement::ONE));
    let ok_pathfinder_invoke_2 = OkTransactionFactory::build(Some(FieldElement::TWO));

    let simulate_flag = vec![SimulationFlagForEstimateFee::SkipValidate];

    let deoxys_estimates = deoxys
        .estimate_fee(
            &vec![ok_deoxys_invoke, ok_deoxys_invoke_1, ok_deoxys_invoke_2],
            simulate_flag.clone(),
            block_number,
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
            simulate_flag.clone(),
            block_number,
        )
        .await
        .unwrap();

    assert_eq!(deoxys_estimates, pathfinder_estimates);
}
