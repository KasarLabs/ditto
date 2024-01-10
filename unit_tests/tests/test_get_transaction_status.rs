#![feature(assert_matches)]

mod common;
use std::{collections::HashMap, assert_matches::assert_matches};

use common::*;
use starknet_core::types::{FieldElement, StarknetError, TransactionStatus};
use starknet_providers::{JsonRpcClient, jsonrpc::HttpTransport, Provider, StarknetErrorWithMessage, ProviderError, MaybeUnknownErrorCode};

///
/// Unit test for `starknet_getTransactionStatus`
/// 
/// purpose: call getTransactionStatus on non-existent transaction hash.
/// fail case: non-existent transaction hash.
/// 
#[rstest]
#[tokio::test]
async fn fail_invalid_transaction(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];

    let response_deoxys = deoxys.get_transaction_status(
        FieldElement::ZERO
    ).await.err();

    assert_matches!(
        response_deoxys,
        Some(ProviderError::StarknetError(StarknetErrorWithMessage {
            message: _,
            code: MaybeUnknownErrorCode::Known(StarknetError::TransactionHashNotFound)
        }))
    );
}

///
/// Unit test for `starknet_getTransactionStatus`
/// 
/// purpose: call getTransactionStatus on transaction which has been accepted on L1.
/// success case: retrieved transaction has been accepted on L1.
/// 
#[rstest]
#[tokio::test]
async fn work_transaction(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];

    let response_deoxys = deoxys.get_transaction_status(
        FieldElement::from_hex_be(TRANSACTION_INVOKE).unwrap()
    ).await.expect("Error waiting for response from Deoxys client");

    assert_matches!(
        response_deoxys,
        TransactionStatus::AcceptedOnL1(_)
    );
}