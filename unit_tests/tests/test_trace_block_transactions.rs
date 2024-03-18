#![feature(assert_matches)]

mod common;
use common::*;

use rand::Rng;
use std::sync::Arc;
use tokio::task::JoinSet;
use std::assert_matches::assert_matches;

use starknet_core::types::{BlockId, BlockTag,FieldElement, StarknetError};
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
        let is_correct_error = checking_error_format(&error, StarknetError::BlockNotFound);

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

//This test may crash because if 2 clients doesnt exactly have the same computation time, the trace will be different
#[rstest]
#[tokio::test]
async fn works_ok_for_pending_block(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
) {
    let mut set = JoinSet::new();
    let arc_deoxys = Arc::new(deoxys);
    let arc_pathfinder = Arc::new(pathfinder);

    let clone_deoxys = Arc::clone(&arc_deoxys);
    set.spawn(async move {
        clone_deoxys
            .trace_block_transactions(BlockId::Tag(BlockTag::Pending))
            .await
            .expect("Error waiting for response from Deoxys node")
    });

    let clone_pathfinder = Arc::clone(&arc_pathfinder);
    set.spawn(async move {
        clone_pathfinder
            .trace_block_transactions(BlockId::Tag(BlockTag::Pending))
            .await
            .expect("Error waiting for response from Pathfinder node")
    });

    let mut deoxys_result = None;
    let mut pathfinder_result = None;

    while let Some(result) = set.join_next().await {
        match result {
            Ok(response) => {
                if deoxys_result.is_none() {
                    deoxys_result = Some(response);
                } else if pathfinder_result.is_none() {
                    pathfinder_result = Some(response);
                }
            },
            Err(e) => panic!("Task panicked or encountered an error: {:?}", e),
        }
    }

    println!("response_deoxys: {:?}", deoxys_result.clone().expect("Deoxys result not found"));
    println!("response_pathfinder: {:?}", pathfinder_result.clone().expect("Pathfinder result not found"));
    assert_eq!(deoxys_result, pathfinder_result, "Responses from Deoxys and Pathfinder do not match");
}
