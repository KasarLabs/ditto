mod common;
use common::*;

use starknet_core::types::SyncStatusType;
use starknet_providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider};
use std::collections::HashMap;
use colored::*;

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

    let response_deoxys = deoxys
        .syncing()
        .await
        .expect("Error while getting sync status from deoxys node");

    let response_pathfinder = pathfinder
        .syncing()
        .await
        .expect("Error while getting sync status from pathfinder node");

    assert_sync_status(response_deoxys, response_pathfinder);
}

/// Assert and print detailed differences between two SyncStatusType
fn assert_sync_status(a: SyncStatusType, b: SyncStatusType) {
    if !compare_sync_status(&a, &b) {
        println!("{}", "Sync status mismatch detected\n".red().bold());
        println!("Deoxys: {}", format!("{:?}\n", a).cyan().bold());
        println!("Pathfinder: {}", format!("{:?}\n", b).magenta().bold());

        if let SyncStatusType::Syncing(a_sync) = &a {
            if let SyncStatusType::Syncing(b_sync) = &b {
                if a_sync.current_block_num != b_sync.current_block_num {
                    println!(
                        "{} {} != {}",
                        "Current block number mismatch:".red(),
                        a_sync.current_block_num.to_string().yellow().bold(),
                        b_sync.current_block_num.to_string().yellow().bold()
                    );
                }
                if a_sync.current_block_hash != b_sync.current_block_hash {
                    println!(
                        "{} {} != {}",
                        "Current block hash mismatch:".red(),
                        format!("0x{:x}", a_sync.current_block_hash).yellow().bold(),
                        format!("0x{:x}", b_sync.current_block_hash).yellow().bold()
                    );
                }
                if a_sync.highest_block_num != b_sync.highest_block_num {
                    println!(
                        "{} {} != {}",
                        "Highest block number mismatch:".red(),
                        a_sync.highest_block_num.to_string().yellow().bold(),
                        b_sync.highest_block_num.to_string().yellow().bold()
                    );
                }
                if a_sync.highest_block_hash != b_sync.highest_block_hash {
                    println!(
                        "{} {} != {}",
                        "Highest block hash mismatch:".red(),
                        format!("0x{:x}", a_sync.highest_block_hash).yellow().bold(),
                        format!("0x{:x}", b_sync.highest_block_hash).yellow().bold()
                    );
                }
                if a_sync.current_block_num != b_sync.current_block_num {
                    println!("{}", "Mismatch skipped since both node does not have the same height".green().bold());
                } else if matches!(b, SyncStatusType::NotSyncing) {
                    panic!("{}", "\nstarknet_syncing mismatch detected".red().bold());
                }
            } else if let SyncStatusType::NotSyncing = &b {
                println!("{}", "\nMismatch skipped since node B is not syncing.".green().bold());
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
