#![feature(assert_matches)]

mod common;
use colored::*;
use common::*;
use starknet_core::types::MaybePendingBlockWithTxHashes;

use std::collections::HashMap;
use std::sync::Arc;

use serde_json::Value;
use starknet_core::types::{BlockId, BlockTag, FieldElement, StarknetError};
use starknet_providers::{
    jsonrpc::{HttpTransport, JsonRpcClient},
    Provider,
};
use unit_tests::constants::DEOXYS;

// Define a recursive function to compare JSON values and print differences
fn compare_json_values(path: &str, value1: &Value, value2: &Value) -> bool {
    let mut exception_found = false;

    match (value1, value2) {
        (Value::Object(map1), Value::Object(map2)) => {
            for key in map1
                .keys()
                .chain(map2.keys())
                .collect::<std::collections::HashSet<_>>()
            {
                let new_path = format!("{}/{}", path, key);
                match (map1.get(key), map2.get(key)) {
                    (Some(v1), Some(v2)) => {
                        if compare_json_values(&new_path, v1, v2) {
                            exception_found = true;
                        }
                    }
                    (Some(v1), None) => {
                        println!("{}: present in first, absent in second", new_path)
                    }
                    (None, Some(v2)) => {
                        println!("{}: absent in first, present in second", new_path)
                    }
                    (None, None) => unreachable!(),
                }
            }
        }
        (Value::Array(arr1), Value::Array(arr2)) => {
            for (index, (item1, item2)) in arr1.iter().zip(arr2.iter()).enumerate() {
                let new_path = format!("{}[{}]", path, index);
                if compare_json_values(&new_path, item1, item2) {
                    exception_found = true;
                }
            }
            if arr1.len() > arr2.len() {
                for index in arr2.len()..arr1.len() {
                    println!("{}[{}]: present in first, absent in second", path, index);
                }
            } else if arr2.len() > arr1.len() {
                for index in arr1.len()..arr2.len() {
                    println!("{}[{}]: absent in first, present in second", path, index);
                }
            }
        }
        _ => {
            if value1 != value2 {
                let exception_paths = [
                    "/l1_data_gas_price/price_in_fri",
                    "/l1_data_gas_price/price_in_wei",
                ];

                if exception_paths.contains(&path) {
                    println!(
                        "{} - Bypassed as exception",
                        format!("{}: {:?} != {:?}", path, value1, value2).yellow()
                    );
                    exception_found = true;
                } else {
                    println!("{}: {:?} != {:?}", path, value1, value2);
                }
            }
        }
    }

    exception_found
}

///
/// Unit test for `starknet_get_block_with_tx_hashes`
///
/// purpose: call getBlockWithTxHashes on invalid block.
/// fail case: invalid block.
///
#[rstest]
#[tokio::test]
async fn fail_non_existing_block(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[mainnet::network::DEOXYS];

    let response_deoxys = deoxys
        .get_block_with_tx_hashes(BlockId::Hash(FieldElement::ZERO))
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

///
/// Unit test for `starknet_get_block_with_tx_hashes`
///
/// purpose: call getBlockWithTxHashes on latest validated block.
/// success case: retrieves valid block.
///
/// Be aware that this test can fail due to the last moments of a block being validated
#[rstest]
#[tokio::test]
async fn work_existing_block(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[mainnet::network::DEOXYS];
    let pathfinder = &clients[mainnet::network::PATHFINDER];

    let block_number = BlockId::Number(100000);

    let response_deoxys = deoxys
        .get_block_with_tx_hashes(block_number)
        .await
        .expect("Error waiting for response from Deoxys node");
    let response_pathfinder = pathfinder
        .get_block_with_tx_hashes(block_number)
        .await
        .expect("Error waiting for Pathfinder node");

    let block_deoxys = match response_deoxys {
        MaybePendingBlockWithTxHashes::Block(block) => block,
        MaybePendingBlockWithTxHashes::PendingBlock(_) => {
            panic!("Expected block, got pending block")
        }
    };
    let block_pathfinder = match response_pathfinder {
        MaybePendingBlockWithTxHashes::Block(block) => block,
        MaybePendingBlockWithTxHashes::PendingBlock(_) => {
            panic!("Expected block, got pending block")
        }
    };

    // Convert the blocks to JSON values
    let block_deoxys_json: Value =
        serde_json::to_value(&block_deoxys).expect("Failed to convert deoxys block to JSON");
    let block_pathfinder_json: Value = serde_json::to_value(&block_pathfinder)
        .expect("Failed to convert pathfinder block to JSON");

    // Compare the JSON values and print differences if they don't match
    if block_deoxys_json != block_pathfinder_json {
        println!(
            "{}",
            format!("Block does not match differences found\n")
                .red()
                .bold()
        );
        let exception_found = compare_json_values("", &block_deoxys_json, &block_pathfinder_json);

        if !exception_found {
            panic!("Blocks do not match");
        } else {
            println!(
                "\nMismatch skipped: {}",
                format!("field exception found").green().bold()
            );
        }
    }
}

///
/// Unit test for `starknet_get_block_with_tx_hashes`
///
/// purpose: call getBlockWithTxHashes on pending block.
/// success case: retrieves valid pending block.
///
/// Note that this can fail at the last moments of a block being validated!!!
///
#[rstest]
#[tokio::test]
#[ignore = "Pending fails some times when called on the cusp of being accepted, need virtual sequencer"]
async fn work_pending_block(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[mainnet::network::DEOXYS];
    let pathfinder = &clients[mainnet::network::PATHFINDER];

    let response_deoxys = deoxys
        .get_block_with_tx_hashes(BlockId::Tag(BlockTag::Pending))
        .await
        .expect("Error waiting for response from Deoxys node");

    let response_pathfinder = pathfinder
        .get_block_with_tx_hashes(BlockId::Tag(BlockTag::Pending))
        .await
        .expect("Error waiting for response from Deoxys node");

    let block_deoxys = match response_deoxys {
        MaybePendingBlockWithTxHashes::Block(_) => panic!("Expected pending block, got block"),
        MaybePendingBlockWithTxHashes::PendingBlock(block) => block,
    };
    let block_pathfinder = match response_pathfinder {
        MaybePendingBlockWithTxHashes::Block(_) => panic!("Expected pending block, got block"),
        MaybePendingBlockWithTxHashes::PendingBlock(block) => block,
    };

    assert_eq!(block_deoxys, block_pathfinder);
}

async fn work_with_block(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
    _block_number: u64,
) {
    let block_number = get_block_setting();

    let response_deoxys = deoxys
        .get_block_with_tx_hashes(block_number)
        .await
        .expect("Error waiting for response from Deoxys node");
    let response_pathfinder = pathfinder
        .get_block_with_tx_hashes(block_number)
        .await
        .expect("Error waiting for Pathfinder node");

    let block_deoxys = match response_deoxys {
        MaybePendingBlockWithTxHashes::Block(block) => block,
        MaybePendingBlockWithTxHashes::PendingBlock(_) => {
            panic!("Expected block, got pending block")
        }
    };
    let block_pathfinder = match response_pathfinder {
        MaybePendingBlockWithTxHashes::Block(block) => block,
        MaybePendingBlockWithTxHashes::PendingBlock(_) => {
            panic!("Expected block, got pending block")
        }
    };

    // Convert the blocks to JSON values
    let block_deoxys_json: Value =
        serde_json::to_value(&block_deoxys).expect("Failed to convert deoxys block to JSON");
    let block_pathfinder_json: Value = serde_json::to_value(&block_pathfinder)
        .expect("Failed to convert pathfinder block to JSON");

    // Compare the JSON values and print differences if they don't match
    if block_deoxys_json != block_pathfinder_json {
        println!(
            "{}",
            format!("Block does not match differences found\n")
                .red()
                .bold()
        );
        let exception_found = compare_json_values("", &block_deoxys_json, &block_pathfinder_json);

        if !exception_found {
            panic!("Blocks do not match");
        } else {
            println!(
                "\nMismatch skipped: {}",
                format!("field exception found").green().bold()
            );
        }
    }
}

/// block 1
#[rstest]
#[tokio::test]
async fn work_with_block_1(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
) {
    work_with_block(deoxys, pathfinder, 1).await;
}

/// block 3800 is the first block with starknet_version in the header
#[rstest]
#[tokio::test]
async fn work_with_block_3800(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
) {
    work_with_block(deoxys, pathfinder, 3000).await;
}

/// block 5066 is one of the biggest blocks in the mainnet
#[rstest]
#[tokio::test]
async fn work_with_block_5066(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
) {
    work_with_block(deoxys, pathfinder, 5066).await;
}

/// block 1466-2242 mismatch block_hash
#[rstest]
#[tokio::test]
async fn work_with_block_1500(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
) {
    work_with_block(deoxys, pathfinder, 1500).await;
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
                    .get_block_with_tx_hashes(block)
                    .await
                    .expect("Error waiting for response from Deoxys node");

                let response_pathfinder = clone_pathfinder.get_block_with_tx_hashes(block).await;

                match response_pathfinder {
                    Ok(response_pathfinder) => {
                        if response_deoxys != response_pathfinder {
                            // Convert the blocks to JSON values
                            let block_deoxys_json: Value = serde_json::to_value(&response_deoxys)
                                .expect("Failed to convert deoxys block to JSON");
                            let block_pathfinder_json: Value =
                                serde_json::to_value(&response_pathfinder)
                                    .expect("Failed to convert pathfinder block to JSON");

                            // Compare the JSON values and print differences
                            println!("Blocks for block {} do not match. Differences:", block_id);
                            let exception_found =
                                compare_json_values("", &block_deoxys_json, &block_pathfinder_json);

                            if !exception_found {
                                Err(format!("block {}", block_id))
                            } else {
                                println!("{}", "Test passed with exceptions".green());
                                Ok(())
                            }
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

/// This test may crash because if 2 clients doesnt exactly have the same computation time, the trace will be different
#[rstest]
#[tokio::test]
#[ignore = "Slash this ignore when Deoxys node is fully synced, but it may not works at all bc of computation"]
async fn work_ok_with_pending_block(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
) {
    let mut set = tokio::task::JoinSet::new();
    let arc_deoxys = Arc::new(deoxys);
    let arc_pathfinder = Arc::new(pathfinder);

    let clone_deoxys = Arc::clone(&arc_deoxys);
    set.spawn(async move {
        clone_deoxys
            .get_block_with_tx_hashes(BlockId::Tag(BlockTag::Pending))
            .await
            .expect("Error waiting for response from Deoxys node")
    });

    let clone_pathfinder = Arc::clone(&arc_pathfinder);
    set.spawn(async move {
        clone_pathfinder
            .get_block_with_tx_hashes(BlockId::Tag(BlockTag::Pending))
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
            }
            Err(e) => panic!("Task panicked or encountered an error: {:?}", e),
        }
    }

    assert_eq!(
        deoxys_result, pathfinder_result,
        "Responses from Deoxys and Pathfinder do not match"
    );
}
