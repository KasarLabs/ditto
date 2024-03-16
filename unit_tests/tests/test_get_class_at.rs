#![feature(assert_matches)]

mod common;
use std::{collections::HashMap, io::Read};

use common::*;
use flate2::read::GzDecoder;
use starknet_core::types::{
    contract::legacy::LegacyProgram, BlockId, BlockTag, ContractClass, FieldElement, StarknetError,
};
use starknet_providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider};

///
/// unit test for `starknet_get_class_at`
///
/// purpose: gets contract class for inexistent block.
/// fail case: invalid block address.
///
#[rstest]
#[tokio::test]
async fn fail_non_existing_block(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];

    let response_deoxys = deoxys
        .get_class_at(
            BlockId::Hash(FieldElement::ZERO),
            FieldElement::from_hex_be(CONTRACT_ACCOUNT).unwrap(),
        )
        .await;

    assert!(
        response_deoxys.is_err(),
        "Expected an error, but got a result"
    );

    if let Err(error) = response_deoxys {
        let is_correct_error = checking_error_format(&error, StarknetError::BlockNotFound);

        assert!(
            is_correct_error,
            "Expected BlockNotFound error, but got a different error"
        );
    }
}

///
/// unit test for `starknet_get_class_at`
///
/// purpose: gets contract class for inexistent contract.
/// fail case: invalid contract address.
///
#[rstest]
#[tokio::test]
async fn fail_non_existing_contract(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];

    let response_deoxys = deoxys
        .get_class_at(BlockId::Tag(BlockTag::Latest), FieldElement::ZERO)
        .await
        .err();

    assert!(
        response_deoxys.is_some(),
        "Expected an error, but got a result"
    );

    let is_correct_error = checking_error_format(
        response_deoxys.as_ref().unwrap(),
        StarknetError::ContractNotFound,
    );

    assert!(
        is_correct_error,
        "Expected ContractNotFound error, but got a different error"
    );
}

///
/// unit test for `starknet_get_class_at`
///
/// purpose: gets legacy contract and extracts it's data.
/// success case: should retrieve contract and decompress it to a valid json string.
#[rstest]
#[tokio::test]
async fn work_contract_v0(
    clients: HashMap<String, JsonRpcClient<HttpTransport>>,
) -> anyhow::Result<()> {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let response_deoxys = deoxys
        .get_class_at(
            BlockId::Number(BLOCK_LEGACY),
            FieldElement::from_hex_be(CONTRACT_LEGACY).unwrap(),
        )
        .await
        .expect("Error waiting for response from Deoxys client");

    let response_pathfinder = pathfinder
        .get_class_at(
            BlockId::Number(BLOCK_LEGACY),
            FieldElement::from_hex_be(CONTRACT_LEGACY).unwrap(),
        )
        .await
        .expect("Error waiting for response from Pathfinder client");

    let mut json_deoxys: String = String::new();
    match &response_deoxys {
        ContractClass::Sierra(_) => panic!("Expected legacy contract"),
        ContractClass::Legacy(contract) => {
            let mut decoder = GzDecoder::new(&contract.program[..]);
            decoder.read_to_string(&mut json_deoxys)?;

            // makes sure json_deoxys is a valid json string
            serde_json::from_str::<LegacyProgram>(&json_deoxys)?;
        }
    }

    let mut json_pathfinder = String::new();
    match &response_pathfinder {
        ContractClass::Sierra(_) => panic!("Expected legacy contract"),
        ContractClass::Legacy(contract) => {
            let mut decoder = GzDecoder::new(&contract.program[..]);
            decoder.read_to_string(&mut json_pathfinder)?;
        }
    }

    assert_eq!(json_deoxys, json_pathfinder);
    assert_eq!(response_deoxys, response_pathfinder);

    anyhow::Ok(())
}

///
/// unit test for `starknet_get_class_at`
///
/// purpose: gets Cairo v1 contract and extracts it's data.
/// success case: should retrieve contract correctly.
///
#[rstest]
#[tokio::test]
async fn work_contract_v1(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
) {
    let response_deoxys = deoxys
        .get_class_at(
            BlockId::Tag(BlockTag::Latest),
            FieldElement::from_hex_be(CONTRACT_ACCOUNT).unwrap(),
        )
        .await
        .expect("Error waiting for response from Deoxys client");

    let response_pathfinder = pathfinder
        .get_class_at(
            BlockId::Tag(BlockTag::Latest),
            FieldElement::from_hex_be(CONTRACT_ACCOUNT).unwrap(),
        )
        .await
        .expect("Error waiting for response from Pathfinder client");

    assert_eq!(response_deoxys, response_pathfinder);
}
