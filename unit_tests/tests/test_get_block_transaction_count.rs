#![feature(assert_matches)]

/// TODO test on a block without transactions
mod common;
use common::*;

use rstest::rstest;
use starknet_core::types::{BlockId, BlockTag, FieldElement, StarknetError};
use starknet_providers::{
    jsonrpc::{HttpTransport, JsonRpcClient},
    Provider,
};
use std::collections::HashMap;
use std::sync::Arc;
use unit_tests::constants::{DEOXYS, PATHFINDER};

#[rstest]
#[tokio::test]
async fn fail_non_existing_block(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];

    let response_deoxys = deoxys
        .get_block_transaction_count(BlockId::Hash(FieldElement::ZERO))
        .await
        .err();

    assert!(
        response_deoxys.is_some(),
        "Expected an error, but got a result"
    );

    let is_correct_error = checking_error_format(
        response_deoxys.as_ref().unwrap(),
        StarknetError::BlockNotFound,
    );

    assert!(
        is_correct_error,
        "Expected BlockNotFound error, but got a different error"
    );
}

#[rstest]
#[tokio::test]
async fn work_with_latest_block(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let block_tag = BlockId::Tag(BlockTag::Latest);

    let response_deoxys = deoxys
        .get_block_transaction_count(block_tag)
        .await
        .expect("Error waiting for response from Deoxys node");

    let response_pathfinder = pathfinder
        .get_block_transaction_count(block_tag)
        .await
        .expect("Error waiting for response from Pathfinder node");

    assert_eq!(response_deoxys, response_pathfinder);
}

async fn work_with_block(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
    block_number: u64,
) {
    let block_number = BlockId::Number(block_number);

    let response_deoxys = deoxys
        .get_block_transaction_count(block_number)
        .await
        .expect("Error waiting for response from Deoxys node");

    let response_pathfinder = pathfinder
        .get_block_transaction_count(block_number)
        .await
        .expect("Error waiting for response from Pathfinder node");

    assert_eq!(response_deoxys, response_pathfinder);
}

#[rstest]
#[tokio::test]
async fn work_with_block_1(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
) {
    work_with_block(deoxys, pathfinder, 1).await;
}

#[rstest]
#[tokio::test]
async fn work_with_block_1_hash(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let block_hash = BlockId::Hash(
        FieldElement::from_hex_be(
            "0x2a70fb03fe363a2d6be843343a1d81ce6abeda1e9bd5cc6ad8fa9f45e30fdeb",
        )
        .expect("Error parsing block hash"),
    );

    let response_deoxys = deoxys
        .get_block_transaction_count(block_hash)
        .await
        .expect("Error waiting for response from Deoxys node");

    let response_pathfinder = pathfinder
        .get_block_transaction_count(block_hash)
        .await
        .expect("Error waiting for response from Pathfinder node");

    assert_eq!(response_deoxys, response_pathfinder);
}

#[rstest]
#[tokio::test]
async fn work_with_block_5066(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
) {
    work_with_block(deoxys, pathfinder, 1).await;
}

#[rstest]
#[tokio::test]
async fn work_with_block_100_000(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
) {
    work_with_block(deoxys, pathfinder, 100_000).await;
}

#[rstest]
#[tokio::test]
async fn work_with_block_100_000_hash(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    if *MAX_BLOCK < 100_000 {
        return;
    }

    let block_hash = BlockId::Hash(
        FieldElement::from_hex_be(
            "0x4f45f870c79f7656c5d7c3c2c28ca0c2fe7206f22f56ac2183f81de521ab340",
        )
        .expect("Error parsing block hash"),
    );

    let response_deoxys = deoxys
        .get_block_transaction_count(block_hash)
        .await
        .expect("Error waiting for response from Deoxys node");

    let response_pathfinder = pathfinder
        .get_block_transaction_count(block_hash)
        .await
        .expect("Error waiting for response from Pathfinder node");

    assert_eq!(response_deoxys, response_pathfinder);
}

#[rstest]
#[tokio::test]
async fn work_loop(deoxys: JsonRpcClient<HttpTransport>, pathfinder: JsonRpcClient<HttpTransport>) {
    let arc_deoxys = Arc::new(deoxys);
    let arc_pathfinder = Arc::new(pathfinder);
    let parallels_queries = 10;
    let mut diff = false;

    for block_group in (0..=100).step_by(parallels_queries) {
        let mut set = tokio::task::JoinSet::new();
        for offset in 0..parallels_queries {
            let block_id = (block_group + offset) as u64;
            let block = BlockId::Number(block_id);
            let clone_deoxys = Arc::clone(&arc_deoxys);
            let clone_pathfinder = Arc::clone(&arc_pathfinder);
            set.spawn(async move {
                let response_deoxys = clone_deoxys
                    .get_block_transaction_count(block)
                    .await
                    .expect("Error waiting for response from Deoxys node");

                let response_pathfinder = clone_pathfinder.get_block_transaction_count(block).await;

                match response_pathfinder {
                    Ok(response_pathfinder) => {
                        if response_deoxys != response_pathfinder {
                            Err(format!("block {}", block_id))
                        } else {
                            Ok(())
                        }
                    }
                    Err(e) => Err(format!("Error pathfinder: {}", e)),
                }
            });
        }
        while let Some(result) = set.join_next().await {
            match result {
                Ok(result) => match result {
                    Ok(_) => {}
                    Err(e) => {
                        println!("{}", e);
                        diff = true;
                    }
                },
                Err(e) => {
                    panic!("{}", e);
                }
            }
        }
    }
    assert_eq!(diff, false);
}
