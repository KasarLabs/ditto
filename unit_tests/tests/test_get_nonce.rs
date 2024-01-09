#![feature(assert_matches)]

mod common;
use std::{collections::HashMap, assert_matches::assert_matches};

use common::*;
use starknet_core::types::{BlockId, FieldElement, StarknetError, BlockTag};
use starknet_providers::{JsonRpcClient, jsonrpc::HttpTransport, Provider, ProviderError, StarknetErrorWithMessage, MaybeUnknownErrorCode};

/**
 * Test for RPC call starknet_getNonce.
 * 
 * *What is a NONCE?*
 * 
 * A nonce is a unique identifier attributed to a starknet transaction, guaranteeing it cannot be added to a
 * block multiple times. As of writing this, Starknet nonces are **sequential**, which is to say that the nonce
 * in a new transaction must follow that of the previous transaction from the same account. The concept of a
 * nonce on Starknet should not be confused with how nonces are used on other blockchains such as Bitcoin as
 * part of proof-of-work.
 * 
 * More documentation can be found in [the Starknet Book](https://book.starknet.io/ch03-01-01-transactions-lifecycle.html#nonces-in-starknet)
 * 
 * @Trantorian1 09-01-2024
 */

#[rstest]
#[tokio::test]
async fn fail_non_existing_block(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];

    let response_deoxys = deoxys.get_nonce(
        BlockId::Hash(FieldElement::ZERO), 
        FieldElement::from_hex_be(STARKGATE_ETH_CONTRACT_ADDR).unwrap()
    ).await.err();

    assert_matches!(
        response_deoxys,
        Some(ProviderError::StarknetError(StarknetErrorWithMessage {
            message: _,
            code: MaybeUnknownErrorCode::Known(StarknetError::BlockNotFound)
        }))
    );
}

#[rstest]
#[tokio::test]
async fn fail_non_existing_contract(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];

    let response_deoxys = deoxys.get_nonce(
        BlockId::Tag(BlockTag::Latest), 
        FieldElement::from_hex_be(INVALID_CONTRACT_ADDR).unwrap()
    ).await.err();

    assert_matches!(
        response_deoxys,
        Some(ProviderError::StarknetError(StarknetErrorWithMessage {
            message: _,
            code: MaybeUnknownErrorCode::Known(StarknetError::ContractNotFound)
        }))
    );
}

