mod common;
use common::*;

use colored::*;
use starknet_core::types::SyncStatusType;
use starknet_providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider};
use std::collections::HashMap;

///
/// Unit test for `starknet_syncing`
///
/// purpose: returns starknet sync status
/// success case: sync status matches between providers (NOT DETERMINISTIC)
///
#[rstest]
#[tokio::test]
async fn syncing(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];
    let node_c = &clients[JUNO];

    let response_deoxys = deoxys
        .syncing()
        .await
        .expect("Error while getting sync status from deoxys node");

    let response_pathfinder = pathfinder
        .syncing()
        .await
        .expect("Error while getting sync status from pathfinder node");

    let response_node_c = node_c
        .syncing()
        .await
        .expect("Error while getting sync status from juno node");

    assert_sync_status(response_deoxys, response_pathfinder, response_node_c);
}

/// Assert and print detailed differences between three SyncStatusType
fn assert_sync_status(a: SyncStatusType, b: SyncStatusType, c: SyncStatusType) {
    let ab_sync_status_match = compare_sync_status(&a, &b);
    let bc_sync_status_match = compare_sync_status(&b, &c);
    let ca_sync_status_match = compare_sync_status(&c, &a);

    if !ab_sync_status_match || !bc_sync_status_match || !ca_sync_status_match {
        println!("{}", "Sync status mismatch detected\n".red().bold());
        println!("- Deoxys: {}", format!("{:?}", a).cyan().bold());
        println!("- Pathfinder: {}", format!("{:?}", b).magenta().bold());
        println!("- Juno: {}\n", format!("{:?}", c).green().bold());

        let nodes = vec![("Deoxys", &a), ("Pathfinder", &b), ("Juno", &c)];
        for i in 0..nodes.len() {
            for j in (i + 1)..nodes.len() {
                let (name1, status1) = &nodes[i];
                let (name2, status2) = &nodes[j];
                if !compare_sync_status(status1, status2) {
                    match (status1, status2) {
                        (SyncStatusType::Syncing(sync1), SyncStatusType::Syncing(sync2)) => {
                            if sync1.current_block_num != sync2.current_block_num {
                                println!(
                                    "{}: {} {} != {} {}",
                                    "Current block number mismatch".red(),
                                    name1,
                                    sync1.current_block_num.to_string().yellow().bold(),
                                    sync2.current_block_num.to_string().yellow().bold(),
                                    name2
                                );
                            }
                            if sync1.current_block_hash != sync2.current_block_hash {
                                println!(
                                    "{}: {} {} != {} {}",
                                    "Current block hash mismatch:".red(),
                                    name1,
                                    format!("0x{:x}", sync1.current_block_hash).yellow().bold(),
                                    format!("0x{:x}", sync2.current_block_hash).yellow().bold(),
                                    name2
                                );
                            }
                            if sync1.highest_block_num != sync2.highest_block_num {
                                println!(
                                    "{}: {} {} != {}",
                                    "Highest block number mismatch:".red(),
                                    name1,
                                    sync1.highest_block_num.to_string().yellow().bold(),
                                    sync2.highest_block_num.to_string().yellow().bold()
                                );
                            }
                            if sync1.highest_block_hash != sync2.highest_block_hash {
                                println!(
                                    "{}: {} {} != {} {}",
                                    "Highest block hash mismatch:".red(),
                                    name1,
                                    format!("0x{:x}", sync1.highest_block_hash).yellow().bold(),
                                    format!("0x{:x}", sync2.highest_block_hash).yellow().bold(),
                                    name2
                                );
                            }
                            if sync1.current_block_num != sync2.current_block_num {
                                println!(
                                    "Mismatch skipped: {}",
                                    "Nodes are not on the same height".green().bold()
                                );
                            }
                        }
                        (SyncStatusType::Syncing(_), SyncStatusType::NotSyncing) => {
                            println!(
                                "Mismatch skipped: {}",
                                format!("Node {} is not syncing.", name2).green().bold()
                            );
                        }
                        (SyncStatusType::NotSyncing, SyncStatusType::Syncing(_)) => {
                            println!(
                                "Mismatch skipped: {}",
                                format!("Node {} is not syncing.", name1).green().bold()
                            );
                        }
                        _ => {
                            panic!(
                                "Mismatch detected: {}",
                                "starknet_syncing mismatch".red().bold()
                            );
                        }
                    }
                }
            }
        }
    }
}

/// compare 2 SyncStatus, only fields corresponding to current and highest block are compared
/// because the other fields are not deterministic and depend on restart of the node
fn compare_sync_status(a: &SyncStatusType, b: &SyncStatusType) -> bool {
    match (a, b) {
        (SyncStatusType::Syncing(a), SyncStatusType::Syncing(b)) => {
            a.current_block_num == b.current_block_num
                && a.current_block_hash == b.current_block_hash
                && a.highest_block_num == b.highest_block_num
                && a.highest_block_hash == b.highest_block_hash
        }
        (SyncStatusType::NotSyncing, SyncStatusType::NotSyncing) => true,
        _ => false,
    }
}
