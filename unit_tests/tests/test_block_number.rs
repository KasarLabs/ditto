mod common;
use common::*;

use std::collections::HashMap;

use colored::*;
use starknet_providers::{
    jsonrpc::{HttpTransport, JsonRpcClient},
    Provider,
}; // Add this import for colored output

///
/// Unit test for `starknet_blockNumber`
///
/// purpose: call blockNumber on latest block.
/// success case: must return valid non-zero block number.
///
#[rstest]
#[tokio::test]
async fn work_existing_block(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[mainnet::network::DEOXYS];
    let pathfinder = &clients[mainnet::network::PATHFINDER];
    let juno = &clients[mainnet::network::JUNO];

    let response_deoxys = deoxys
        .block_number()
        .await
        .expect("Deoxys : Error while getting the block number");
    let response_pathfinder = pathfinder
        .block_number()
        .await
        .expect("RPC : Error while getting the block number");
    let response_juno = juno
        .block_number()
        .await
        .expect("Juno : Error while getting the block number");

    assert!(response_deoxys > 0);
    assert!(response_pathfinder > 0);
    assert!(response_juno > 0);

    let mut mismatch = false;

    if response_deoxys != response_pathfinder
        || response_pathfinder != response_juno
        || response_juno != response_deoxys
    {
        mismatch = true;
        println!("{}", "Block number mismatch detected\n".red().bold());
        println!("Deoxys: {}", format!("{}", response_deoxys).cyan().bold());
        println!(
            "Pathfinder: {}",
            format!("{}", response_pathfinder).magenta().bold()
        );
        println!("Juno: {}\n", format!("{}", response_juno).green().bold());

        if response_deoxys != response_pathfinder {
            println!(
                "{} {} != {}",
                "Mismatch between Deoxys and Pathfinder:".red(),
                response_deoxys.to_string().yellow().bold(),
                response_pathfinder.to_string().yellow().bold()
            );
        }
        if response_pathfinder != response_juno {
            println!(
                "{} {} != {}",
                "Mismatch between Pathfinder and Juno:".red(),
                response_pathfinder.to_string().yellow().bold(),
                response_juno.to_string().yellow().bold()
            );
        }
        if response_juno != response_deoxys {
            println!(
                "{} {} != {}",
                "Mismatch between Juno and Deoxys:".red(),
                response_juno.to_string().yellow().bold(),
                response_deoxys.to_string().yellow().bold()
            );
        }
    } else {
        println!("{}", "All nodes have matching block numbers".green().bold());
    }

    if mismatch {
        println!(
            "{}",
            "\nMismatch on Block numbers are skipped since it may not be an error."
                .green()
                .bold()
        );
    }
}
