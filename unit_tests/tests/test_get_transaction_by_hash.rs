#![feature(assert_matches)]

mod common;
use std::{collections::HashMap, assert_matches::assert_matches};

use common::*;
use starknet_core::types::{FieldElement, StarknetError};
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