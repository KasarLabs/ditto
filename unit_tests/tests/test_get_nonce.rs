#![feature(assert_matches)]

mod common;
use std::collections::HashMap;

use common::*;
use starknet_core::types::{BlockId, BlockTag, FieldElement, StarknetError};
use starknet_providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider};

///
/// Test for RPC call starknet_getNonce.
///
/// *What is a NONCE?*
///
/// A nonce is a unique identifier attributed to a starknet transaction, guaranteeing it cannot be added to a
/// block multiple times. As of writing this, Starknet nonces are **sequential**, which is to say that the nonce
/// in a new transaction must follow that of the previous transaction from the same account. The concept of a
/// nonce on Starknet should not be confused with how nonces are used on other blockchains such as Bitcoin as
/// part of proof-of-work.
///
/// More documentation can be found in [the Starknet Book](https://book.starknet.io/ch03-01-01-transactions-lifecycle.html#nonces-in-starknet)
///
/// [Trantorian1](https://github.com/trantorian1) 09-01-2024
///

///
/// Unit test for `starknet_getNonce`
///
/// purpose: call getNonce on invalid block.
/// fail case: invalid block.
///
#[rstest]
#[tokio::test]
async fn fail_non_existing_block(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[mainnet::network::DEOXYS];

    let response_deoxys = deoxys
        .get_nonce(
            BlockId::Hash(FieldElement::ZERO),
            FieldElement::from_hex_be(STARKGATE_ETH_CONTRACT_ADDR).unwrap(),
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
/// Unit test for `starknet_getNonce`
///
/// purpose: call getNonce on invalid contract.
/// fail case: invalid contract.
///
#[rstest]
#[tokio::test]
async fn fail_non_existing_contract(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[mainnet::network::DEOXYS];

    let response_deoxys = deoxys
        .get_nonce(
            BlockId::Tag(BlockTag::Latest),
            FieldElement::from_hex_be(INVALID_CONTRACT_ADDR).unwrap(),
        )
        .await;

    assert!(
        response_deoxys.is_err(),
        "Expected an error, but got a result"
    );

    if let Err(error) = response_deoxys {
        let is_correct_error = checking_error_format(&error, StarknetError::ContractNotFound);

        assert!(
            is_correct_error,
            "Expected ContractNotFound error, but got a different error"
        );
    }
}

// INFO: I guess non-account contracts don't need a nonce since they are only sent once?
// I'm not sure about this one.

///
/// Unit test for `starknet_getNonce`
///
/// purpose: call getNonce on ERC721 contract.
/// success case: must return a nonce of 0.
///
#[rstest]
#[tokio::test]
async fn work_erc721_contract(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[mainnet::network::DEOXYS];

    let block_number = get_block_setting();

    let response_deoxys = deoxys
        .get_nonce(
            block_number,
            FieldElement::from_hex_be(CONTRACT_ERC721).unwrap(),
        )
        .await
        .expect("Error waiting for response from Deoxys node");

    assert_eq!(response_deoxys, FieldElement::ZERO);
}

///
/// Unit test for `starknet_getNonce`
///
/// purpose: call getNonce on ERC20 contract.
/// success case: must return a nonce of 0.
///
#[rstest]
#[tokio::test]
async fn work_erc20_contract(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[mainnet::network::DEOXYS];

    //This contract was created at Block 500192, so need to be synced to this minimum block

    let block_number = get_block_setting();

    let response_deoxys = deoxys
        .get_nonce(
            block_number,
            FieldElement::from_hex_be(CONTRACT_ERC20).unwrap(),
        )
        .await
        .expect("Error waiting for response from Deoxys node");

    assert_eq!(response_deoxys, FieldElement::ZERO);
}

///
/// Unit test for `starknet_getNonce`
///
/// purpose: call getNonce on account contract.
/// success case: must return a non-zero nonce.
///
#[rstest]
#[tokio::test]
async fn work_account_contract(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[mainnet::network::DEOXYS];
    let pathfinder = &clients[mainnet::network::PATHFINDER];

    let block_number = BlockId::Number(55000);

    let response_deoxys = deoxys
        .get_nonce(
            block_number,
            FieldElement::from_hex_be(CONTRACT_ACCOUNT_CAIRO_ZERO).unwrap(),
        )
        .await
        .expect("Error waiting for response from Deoxys node");

    let response_pathfinder = pathfinder
        .get_nonce(
            block_number,
            FieldElement::from_hex_be(CONTRACT_ACCOUNT_CAIRO_ZERO).unwrap(),
        )
        .await
        .expect("Error waiting for response from Pathfinder node");

    assert_ne!(response_deoxys, FieldElement::ZERO);
    assert_eq!(response_deoxys, response_pathfinder);
}

///
/// Unit test for `starknet_getNonce`
///
/// purpose: call getNonce on account proxy contract.
/// success case: must return a non-zero nonce.
///
#[rstest]
#[tokio::test]
async fn work_account_proxy_contract(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[mainnet::network::DEOXYS];
    let pathfinder = &clients[mainnet::network::PATHFINDER];

    let block_number = BlockId::Number(51244);

    let response_deoxys = deoxys
        .get_nonce(
            block_number,
            FieldElement::from_hex_be(CONTRACT_ACCOUNT_PROXY_CAIRO_ZERO).unwrap(),
        )
        .await
        .expect("Error waiting for response from Deoxys node");

    let response_pathfinder = pathfinder
        .get_nonce(
            block_number,
            FieldElement::from_hex_be(CONTRACT_ACCOUNT_PROXY_CAIRO_ZERO).unwrap(),
        )
        .await
        .expect("Error waiting for response from Pathfinder node");

    assert_ne!(response_deoxys, FieldElement::ZERO);
    assert_eq!(response_deoxys, response_pathfinder);
}
