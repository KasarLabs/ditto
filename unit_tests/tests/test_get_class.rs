#![feature(assert_matches)]

mod common;
use anyhow::{anyhow, Result};
use base64::decode;
use common::*;

use starknet_core::types::{BlockId, ContractClass, FieldElement, StarknetError};
use starknet_providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider, ProviderError};
use std::collections::HashMap;

#[rstest]
#[tokio::test]
async fn fail_non_existing_block(clients: HashMap<String, JsonRpcClient<HttpTransport>>) -> Result<()> {
    let deoxys = &clients[DEOXYS];

    let test_contract_class_hash =
        FieldElement::from_hex_be(TEST_CONTRACT_CLASS_HASH_V0).map_err(|e| anyhow!("Invalid Contract Class Hash: {}", e))?;
    let block_id = BlockId::Number(800000);

    match deoxys.get_class(block_id, test_contract_class_hash).await {
        Err(e) => {
            if checking_error_format(&e, StarknetError::BlockNotFound) {
                eprintln!("Error: Block not found for block ID {:?}", block_id);
                Ok(())
            } else {
                panic!("Unexpected error: {:?}", e);
            }
        },
        Ok(_) => panic!("Unexpected success: Class was found when it shouldn't be."),
    }
}

#[rstest]
#[tokio::test]
async fn fail_non_existing_class_hash(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];

    let unknown_contract_class_hash =
        FieldElement::from_hex_be("0x4269DEADBEEF").expect("Invalid Contract class hash");

    let response_deoxys = deoxys
        .get_class(BlockId::Number(0), unknown_contract_class_hash)
        .await;

    assert!(
        response_deoxys.is_err(),
        "Expected an error, but got a result"
    );

    if let Err(error) = response_deoxys {
        let is_correct_error = checking_error_format(&error, StarknetError::ClassHashNotFound);

        assert!(
            is_correct_error,
            "Expected ClassHashNotFound error, but got a different error: {:?}",
            error
        );
    } else {
        panic!("Unexpected success: Class hash was found when it shouldn't be.");
    }
}

#[rstest]
#[tokio::test]
async fn work_ok_retrieving_class_for_contract_version_0(
    clients: HashMap<String, JsonRpcClient<HttpTransport>>,
) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let test_contract_class_hash =
        FieldElement::from_hex_be(TEST_CONTRACT_CLASS_HASH_V0).expect("Invalid Contract Class Hash");

    let deoxys_class = deoxys
        .get_class(BlockId::Number(50000), test_contract_class_hash)
        .await
        .unwrap();

    let pathfinder_class = pathfinder
        .get_class(BlockId::Number(50000), test_contract_class_hash)
        .await
        .unwrap();

    if let (ContractClass::Legacy(deoxys_legacy), ContractClass::Legacy(pathfinder_legacy)) = (deoxys_class, pathfinder_class) {
        assert_eq!(deoxys_legacy.entry_points_by_type, pathfinder_legacy.entry_points_by_type);
        assert_eq!(deoxys_legacy.abi, pathfinder_legacy.abi);
        
        let deoxys_program = decode(&deoxys_legacy.program).expect("Failed to decode base64 program");
        let pathfinder_program = decode(&pathfinder_legacy.program).expect("Failed to decode base64 program");
        
        assert_eq!(deoxys_program, pathfinder_program);
    } else {
        panic!("Contract classes are not of the Legacy variant");
    }
}

#[rstest]
#[tokio::test]
async fn work_ok_retrieving_class_for_contract_version_1(
    clients: HashMap<String, JsonRpcClient<HttpTransport>>,
) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let test_contract_class_hash = FieldElement::from_hex_be(TEST_CONTRACT_CLASS_HASH_V1)
        .expect("Invalid Contract Class Hash");

    let deoxys_class = deoxys
        .get_class(BlockId::Number(250000), test_contract_class_hash)
        .await
        .unwrap();

    let pathfinder_class = pathfinder
        .get_class(BlockId::Number(250000), test_contract_class_hash)
        .await
        .unwrap();

    assert_eq!(deoxys_class, pathfinder_class);
}
