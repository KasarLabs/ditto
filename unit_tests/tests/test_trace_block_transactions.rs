#![feature(assert_matches)]

mod common;
use common::*;

use rand::Rng;
use std::assert_matches::assert_matches;

use starknet_core::types::{BlockId, FieldElement, StarknetError};
use starknet_providers::{
    jsonrpc::{HttpTransport, JsonRpcClient},
    Provider,
};

#[rstest]
#[tokio::test]
async fn fail_non_existing_block(deoxys: JsonRpcClient<HttpTransport>) {
    let response_deoxys = deoxys
        .trace_block_transactions(BlockId::Hash(FieldElement::ZERO))
        .await;

    assert!(
        response_deoxys.is_err(),
        "Expected an error, but got a result"
    );

    if let Err(error) = response_deoxys {
        let is_correct_error = checking_error_format(
            &error,
            StarknetError::BlockNotFound,
        );

        assert!(
            is_correct_error,
            "Expected BlockNotFound error, but got a different error"
        );
    }
}

#[rstest]
#[tokio::test]
async fn works_ok_for_block_10000(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
) {
    let block_number = BlockId::Number(10000);

    let deoxys_trace = deoxys.trace_block_transactions(block_number).await;
    let _pathfinder_trace = pathfinder.trace_block_transactions(block_number).await;

    assert_matches!(deoxys_trace, _pathfinder_trace);
}

#[rstest]
#[tokio::test]
async fn works_ok_for_block_300000(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
) {
    let block_number = BlockId::Number(300000);

    let deoxys_trace = deoxys.trace_block_transactions(block_number).await;
    let _pathfinder_trace = pathfinder.trace_block_transactions(block_number).await;

    assert_matches!(deoxys_trace, _pathfinder_trace);
}

#[rstest]
#[tokio::test]
async fn works_ok_for_random_block(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
) {
    let mut rng = rand::thread_rng();
    let random_block_number = rng.gen_range(100000..650000);

    let block_number = BlockId::Number(random_block_number);

    let deoxys_trace = deoxys.trace_block_transactions(block_number).await;
    let _pathfinder_trace = pathfinder.trace_block_transactions(block_number).await;
    println!("block choose is: {:?}", block_number);

    assert_matches!(deoxys_trace, _pathfinder_trace);
}
