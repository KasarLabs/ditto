#![feature(assert_matches)]

mod common;
use common::*;

use starknet_core::types::{BlockId, FieldElement, StarknetError};
use starknet_providers::{
    jsonrpc::HttpTransport, JsonRpcClient, MaybeUnknownErrorCode, Provider, ProviderError,
    StarknetErrorWithMessage,
};
use std::assert_matches::assert_matches;
use std::collections::HashMap;

#[rstest]
#[tokio::test]
#[ignore = "Fix failing unwrap due to empty constant"]
async fn fail_non_existing_block(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];

    let test_contract_class_hash =
        FieldElement::from_hex_be(TEST_CONTRACT_CLASS_HASH).expect("Invalid Contract Address");

    assert_matches!(
        deoxys
        .get_class(
            BlockId::Number(100),
            test_contract_class_hash,
        )
        .await,
        Err(ProviderError::StarknetError(StarknetErrorWithMessage { code: MaybeUnknownErrorCode::Known(code), .. })) if code == StarknetError::BlockNotFound
    );
}

#[rstest]
#[tokio::test]
#[ignore = "Fix failing unwrap due to empty constant"]
async fn fail_non_existing_class_hash(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];

    let unknown_contract_class_hash =
        FieldElement::from_hex_be("0x4269DEADBEEF").expect("Invalid Contract classh hash");

    assert_matches!(
        deoxys
        .get_class(
            BlockId::Number(0),
            unknown_contract_class_hash,
        )
        .await,
        Err(ProviderError::StarknetError(StarknetErrorWithMessage { code: MaybeUnknownErrorCode::Known(code), .. })) if code == StarknetError::ClassHashNotFound
    );
}

#[rstest]
#[tokio::test]
#[ignore = "Fix failing unwrap due to empty constant"]
async fn work_ok_retrieving_class_for_contract_version_0(
    clients: HashMap<String, JsonRpcClient<HttpTransport>>,
) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    //TODO: Check this contract class hash to ensure test is valid, must be CAIRO_0
    let test_contract_class_hash =
        FieldElement::from_hex_be(TEST_CONTRACT_CLASS_HASH).expect("Invalid Contract Class Hash");

    let deoxys_class = deoxys
        .get_class(BlockId::Number(0), test_contract_class_hash)
        .await
        .unwrap();

    let pathfinder_class = pathfinder
        .get_class(BlockId::Number(0), test_contract_class_hash)
        .await
        .unwrap();

    assert_eq!(deoxys_class, pathfinder_class);
}

#[rstest]
#[tokio::test]
#[ignore = "Fix failing unwrap due to empty constant"]
async fn work_ok_retrieving_class_for_contract_version_1(
    clients: HashMap<String, JsonRpcClient<HttpTransport>>,
) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    //TODO: Check this contract class hash to ensure test is valid, must be CAIRO_1
    let test_contract_class_hash = FieldElement::from_hex_be(CAIRO_1_ACCOUNT_CONTRACT_CLASS_HASH)
        .expect("Invalid Contract Class Hash");

    let deoxys_class = deoxys
        .get_class(BlockId::Number(0), test_contract_class_hash)
        .await
        .unwrap();

    let pathfinder_class = pathfinder
        .get_class(BlockId::Number(0), test_contract_class_hash)
        .await
        .unwrap();

    assert_eq!(deoxys_class, pathfinder_class);
}
