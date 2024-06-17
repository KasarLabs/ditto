#![feature(assert_matches)]

mod common;
use std::collections::HashMap;

use colored::*;
use common::*;
use starknet_providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider};

///
/// Unit test for `starknet_specversion`
///
/// Purpose: Retrieve the Deoxys node spec version
/// Success case: Spec version should be 0.7.1
///
#[rstest]
#[tokio::test]
async fn test_spec_version_7_1(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];
    let juno = &clients[JUNO];

    let response_deoxys = deoxys
        .spec_version()
        .await
        .expect("Deoxys: Error while getting the block number");
    let response_pathfinder = pathfinder
        .spec_version()
        .await
        .expect("RPC: Error while getting the block number");
    let response_juno = juno
        .spec_version()
        .await
        .expect("Juno: Error while getting the block number");

    assert_eq!(response_deoxys, SPEC_0_7_1, "Deoxys spec version mismatch");
    assert_eq!(
        response_pathfinder, SPEC_0_7_0,
        "Pathfinder spec version mismatch"
    );
    assert_eq!(response_juno, SPEC_0_7_1, "Juno spec version mismatch");

    println!(
        "Spec version matches for all clients: {}",
        format!("0.7.1").green().bold()
    );
}
