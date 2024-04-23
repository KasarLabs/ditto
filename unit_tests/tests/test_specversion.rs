#![feature(assert_matches)]

mod common;
use common::*;
use starknet_providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider};

///
/// Unit test for `starknet_specversion`
///
/// purpose: retrieve the Deoxys node spec version
/// success case: spec version should be 0.7.0
///
#[rstest]
#[tokio::test]
#[logging]
async fn test_specversion(deoxys: JsonRpcClient<HttpTransport>) {
    let response_deoxys = deoxys.spec_version().await.expect(ERR_DEOXYS);

    log::info!("Deoxys RPC spec: {}", response_deoxys);
    assert_eq!(response_deoxys, SPEC_0_7_0);
}
