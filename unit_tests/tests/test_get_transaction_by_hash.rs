#![feature(assert_matches)]

mod common;
use std::{collections::HashMap, assert_matches::assert_matches};

use common::*;
use starknet_core::types::{FieldElement, StarknetError, Transaction};
use starknet_providers::{JsonRpcClient, jsonrpc::HttpTransport, Provider, StarknetErrorWithMessage, ProviderError, MaybeUnknownErrorCode};

#[rstest]
#[tokio::test]
async fn fail_non_existing_transaction(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];

    let response_deoxys = deoxys.get_transaction_by_hash(
        FieldElement::ZERO
    ).await.err();
    
    assert_matches!(
        response_deoxys, 
        Some(ProviderError::StarknetError(StarknetErrorWithMessage {
            message: _,
            code: MaybeUnknownErrorCode::Known(StarknetError::TransactionHashNotFound)
        }
    )));
}

#[rstest]
#[tokio::test]
async fn work_transaction_invoke(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let response_deoxys = deoxys.get_transaction_by_hash(
        FieldElement::from_hex_be(TRANSACTION_INVOKE).unwrap()
    ).await.expect("Error waiting for response from Deoxys node");

    let response_pathfinder = pathfinder.get_transaction_by_hash(
        FieldElement::from_hex_be(TRANSACTION_INVOKE).unwrap()
    ).await.expect("Error waiting for response from Pathfinder node");

    assert_matches!(response_deoxys, Transaction::Invoke(_));
    assert_eq!(response_deoxys, response_pathfinder);
}