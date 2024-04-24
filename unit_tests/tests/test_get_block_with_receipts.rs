#![feature(assert_matches)]

mod common;
use common::*;

use rand::Rng;
use std::assert_matches::assert_matches;
use std::sync::Arc;
use tokio::task::JoinSet;

use starknet_core::types::{BlockId, BlockTag, FieldElement, StarknetError};
use starknet_providers::{
    jsonrpc::{HttpTransport, JsonRpcClient},
    Provider,
};

#[rstest]
#[tokio::test]
async fn fail_non_existing_block(deoxys: JsonRpcClient<HttpTransport>) {
    let response_deoxys = deoxys
        .get_block_with_receipts(BlockId::Hash(FieldElement::ZERO))
        .await;

    assert!(
        response_deoxys.is_err(),
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
async fn works_ok_for_block_1(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
) {
    let block_number = BlockId::Number(1);

    let deoxys_trace = deoxys.get_block_with_receipts(block_number).await;
    let _pathfinder_trace = pathfinder.get_block_with_receipts(block_number).await;

    assert_matches!(deoxys_trace, _pathfinder_trace);
}

#[rstest]
#[tokio::test]
async fn works_ok_for_block_with_l1_handler_tx(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
) {
    let block_number = BlockId::Number(192); //got the first L1HandlerTx at block 192

    let deoxys_trace = deoxys.get_block_with_receipts(block_number).await;
    let _pathfinder_trace = pathfinder.get_block_with_receipts(block_number).await;

    assert_matches!(deoxys_trace, _pathfinder_trace);
}

#[rstest]
#[tokio::test]
async fn works_ok_for_block_5000(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
) {
    let block_number = BlockId::Number(5000);

    let deoxys_trace = deoxys.get_block_with_receipts(block_number).await;
    let _pathfinder_trace = pathfinder.get_block_with_receipts(block_number).await;

    assert_matches!(deoxys_trace, _pathfinder_trace);
}

#[rstest]
#[tokio::test]
async fn works_ok_for_block_10000(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
) {
    let block_number = BlockId::Number(10000);

    let deoxys_trace = deoxys.get_block_with_receipts(block_number).await;
    let _pathfinder_trace = pathfinder.get_block_with_receipts(block_number).await;

    assert_matches!(deoxys_trace, _pathfinder_trace);
}

#[rstest]
#[tokio::test]
async fn works_ok_for_block_100000(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
) {
    let block_number = BlockId::Number(100000);

    let deoxys_trace = deoxys.get_block_with_receipts(block_number).await;
    let _pathfinder_trace = pathfinder.get_block_with_receipts(block_number).await;

    assert_matches!(deoxys_trace, _pathfinder_trace);
}
