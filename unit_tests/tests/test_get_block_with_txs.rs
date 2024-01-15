#![feature(assert_matches)]

/// TODO test on a block withouth transactions
mod common;
use common::*;

use std::{assert_matches::assert_matches, collections::HashMap};

use starknet_core::types::{BlockId, BlockTag, FieldElement, StarknetError};
use starknet_providers::{
    jsonrpc::{HttpTransport, JsonRpcClient},
    Provider, ProviderError,
};
use unit_tests::constants::DEOXYS;

#[require(spec_version = "0.5.1")]
#[rstest]
#[tokio::test]
async fn fail_non_existing_block(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];

    let response_deoxys = deoxys
        .get_block_with_txs(BlockId::Hash(FieldElement::ZERO))
        .await
        .err();

    assert_matches!(
        response_deoxys,
        Some(ProviderError::StarknetError(StarknetError::BlockNotFound))
    );
}

#[require(block_min = "latest", spec_version = "0.5.1")]
#[rstest]
#[tokio::test]
async fn work_with_latest_block(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let block_tag = BlockId::Tag(BlockTag::Latest);

    let response_deoxys = deoxys
        .get_block_with_txs(block_tag)
        .await
        .expect("Error waiting for response from Deoxys node");

    let response_pathfinder = pathfinder
        .get_block_with_txs(block_tag)
        .await
        .expect("Error waiting for response from Pathfinder node");

    assert_eq!(response_deoxys, response_pathfinder);
}

#[require(block_min = 1, spec_version = "0.5.1")]
#[rstest]
#[tokio::test]
async fn work_with_block_one_num(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let block_number = BlockId::Number(1);

    let response_deoxys = deoxys
        .get_block_with_txs(block_number)
        .await
        .expect("Error waiting for response from Deoxys node");

    let response_pathfinder = pathfinder
        .get_block_with_txs(block_number)
        .await
        .expect("Error waiting for response from Pathfinder node");

    assert_eq!(response_deoxys, response_pathfinder);
}

#[require(block_min = 1, spec_version = "0.5.1")]
#[rstest]
#[tokio::test]
async fn work_with_block_one_hash(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let block_hash = BlockId::Hash(
        FieldElement::from_hex_be(
            "0x2a70fb03fe363a2d6be843343a1d81ce6abeda1e9bd5cc6ad8fa9f45e30fdeb",
        )
        .expect("Error parsing block hash"),
    );

    let response_deoxys = deoxys
        .get_block_with_txs(block_hash)
        .await
        .expect("Error waiting for response from Deoxys node");

    let response_pathfinder = pathfinder
        .get_block_with_txs(block_hash)
        .await
        .expect("Error waiting for response from Pathfinder node");

    assert_eq!(response_deoxys, response_pathfinder);
}

#[require(block_min = 100_000, spec_version = "0.5.1")]
#[rstest]
#[tokio::test]
async fn work_with_block_one_hundred_thousand_num(
    clients: HashMap<String, JsonRpcClient<HttpTransport>>,
) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let block_number = BlockId::Number(100000);

    let response_deoxys = deoxys
        .get_block_with_txs(block_number)
        .await
        .expect("Error waiting for response from Deoxys node");

    let response_pathfinder = pathfinder
        .get_block_with_txs(block_number)
        .await
        .expect("Error waiting for response from Pathfinder node");

    assert_eq!(response_deoxys, response_pathfinder);
}

#[require(block_min = 100_000, spec_version = "0.5.1")]
#[rstest]
#[tokio::test]
async fn work_with_block_one_hundred_thousand_hash(
    clients: HashMap<String, JsonRpcClient<HttpTransport>>,
) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let block_hash = BlockId::Hash(
        FieldElement::from_hex_be(
            "0x4f45f870c79f7656c5d7c3c2c28ca0c2fe7206f22f56ac2183f81de521ab340",
        )
        .expect("Error parsing block hash"),
    );

    let response_deoxys = deoxys
        .get_block_with_txs(block_hash)
        .await
        .expect("Error waiting for response from Deoxys node");

    let response_pathfinder = pathfinder
        .get_block_with_txs(block_hash)
        .await
        .expect("Error waiting for response from Pathfinder node");

    assert_eq!(response_deoxys, response_pathfinder);
}
