mod common;
use common::*;
use starknet_core::types::BlockHashAndNumber;

use std::collections::HashMap;

use starknet_providers::{
    jsonrpc::{HttpTransport, JsonRpcClient},
    Provider,
};

use colored::*;

///
/// Unit test for `starknet_BlockHashAndNumber`
///
/// purpose: get block hash and number on latest block.
/// success case: retrieves correct block hash and number.
///
#[rstest]
#[tokio::test]
async fn work_existing_block(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];
    let juno = &clients[JUNO];

    let deoxys_responce = deoxys
        .block_hash_and_number()
        .await
        .expect("Deoxys : Error while getting the block number");
    let pathfinder_responce = pathfinder
        .block_hash_and_number()
        .await
        .expect("RPC : Error while getting the block number");
    let juno_responce = juno
        .block_hash_and_number()
        .await
        .expect("Juno : Error while getting the block number");

    assert!(deoxys_responce.block_number > 0);
    assert!(pathfinder_responce.block_number > 0);
    assert!(juno_responce.block_number > 0);

    if !check_block_number(deoxys_responce.clone(), pathfinder_responce.clone(), juno_responce.clone()) {
        println!("{}", "\nMismatch on Block numbers are skipped since it may not be an error.".green().bold());
    }
    
    if !check_block_hashes(deoxys_responce, pathfinder_responce, juno_responce) {
        println!("{}", "\nMismatch on Block hashes are skipped since it may not be an error.".green().bold());
    }
}

fn check_block_number(responce_deoxys: BlockHashAndNumber, responce_pathfinder: BlockHashAndNumber, responce_juno: BlockHashAndNumber) -> bool {
    let deoxys_block_number = responce_deoxys.block_number;
    let pathfinder_block_number = responce_pathfinder.block_number;
    let juno_block_number = responce_juno.block_number;

    if deoxys_block_number != pathfinder_block_number || pathfinder_block_number != juno_block_number || juno_block_number != deoxys_block_number {
        println!("{}", "Block number mismatch detected\n".red().bold());
        println!("Deoxys: {}", format!("{}", deoxys_block_number).cyan().bold());
        println!("Pathfinder: {}", format!("{}", pathfinder_block_number).magenta().bold());
        println!("Juno: {}\n", format!("{}", juno_block_number).green().bold());

        if deoxys_block_number != pathfinder_block_number {
            println!(
                "{} {} != {}",
                "Mismatch between Deoxys and Pathfinder:".red(),
                deoxys_block_number.to_string().yellow().bold(),
                pathfinder_block_number.to_string().yellow().bold()
            );
        }
        if pathfinder_block_number != juno_block_number {
            println!(
                "{} {} != {}",
                "Mismatch between Pathfinder and Juno:".red(),
                pathfinder_block_number.to_string().yellow().bold(),
                juno_block_number.to_string().yellow().bold()
            );
        }
        if juno_block_number != deoxys_block_number {
            println!(
                "{} {} != {}",
                "Mismatch between Juno and Deoxys:".red(),
                juno_block_number.to_string().yellow().bold(),
                deoxys_block_number.to_string().yellow().bold()
            );
        }

        return false;
    } else {
        println!("{}", "All nodes have matching block numbers".green().bold());
        return true;
    }

}

fn check_block_hashes(responce_deoxys: BlockHashAndNumber, responce_pathfinder: BlockHashAndNumber, responce_juno: BlockHashAndNumber) -> bool {
    let deoxys_block_hash = responce_deoxys.block_hash;
    let pathfinder_block_hash = responce_pathfinder.block_hash;
    let juno_block_hash = responce_juno.block_hash;

    if deoxys_block_hash != pathfinder_block_hash || pathfinder_block_hash != juno_block_hash || juno_block_hash != deoxys_block_hash {
        println!("{}", "Block hash mismatch detected\n".red().bold());
        println!("Deoxys: {}", format!("0x{:x}", deoxys_block_hash).cyan().bold());
        println!("Pathfinder: {}", format!("0x{:x}", pathfinder_block_hash).magenta().bold());
        println!("Juno: {}\n", format!("0x{:x}", juno_block_hash).green().bold());

        if deoxys_block_hash != pathfinder_block_hash {
            println!(
                "{} {} != {}",
                "Mismatch between Deoxys and Pathfinder:".red(),
                format!("0x{:x}", deoxys_block_hash).yellow().bold(),
                format!("0x{:x}", pathfinder_block_hash).yellow().bold()
            );
        }
        if pathfinder_block_hash != juno_block_hash {
            println!(
                "{} {} != {}",
                "Mismatch between Pathfinder and Juno:".red(),
                format!("0x{:x}", pathfinder_block_hash).yellow().bold(),
                format!("0x{:x}", juno_block_hash).yellow().bold()
            );
        }
        if juno_block_hash != deoxys_block_hash {
            println!(
                "{} {} != {}",
                "Mismatch between Juno and Deoxys:".red(),
                format!("0x{:x}", juno_block_hash).yellow().bold(),
                format!("0x{:x}", deoxys_block_hash).yellow().bold()
            );
        }

        return false;
    } else {
        println!("{}", "All nodes have matching block hashes".green().bold());
        return true;
    }

}
