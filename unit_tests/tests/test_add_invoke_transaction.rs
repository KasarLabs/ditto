#![feature(assert_matches)]

mod common;
use common::*;
use starknet_core::types::{
    BroadcastedInvokeTransaction, FieldElement, StarknetError, TransactionStatus,
};
use starknet_providers::{
    jsonrpc::{HttpTransport, JsonRpcClient},
    Provider, ProviderError,
};
use std::assert_matches::assert_matches;
use std::thread;
use std::time::Duration;

/// Test for the `add_invoke_transaction` Deoxys RPC method
/// Submit a new transaction to be added to the chain
///
/// # Arguments
/// * `invoke_transaction` - An invoke transaction,
///     with following fields:
///         * `type` - INVOKE
///         * `sender_address` - The address of the sender
///         * `calldata` - The calldata to send
///         * `max_fee` - The maximum fees sender is willing to pay
///         * `version` - The version of the transaction
///         * `signature` - The transaction signature
///         * `nonce` - The nonce of the transaction
///
/// # Returns
/// * `result` - The result of the transaction submission, with the transaction hash that has been submitted
///
/// # Errors
/// * `invalid_transaction_nonce` - If the transaction nonce is invalid
/// * `insufficient_account_balance` - If the account balance is insufficient
/// * `insufficient_max_fee` - If the max fee is insufficient
/// * `invalid_transaction_nonce` - If the transaction nonce is invalid
/// * `validation_failure` - If the transaction validation fails
/// * `non_account` - If the sender address is not a valid account
/// * `duplicate_transaction` - If a transaction with same params already exists
/// * `unsupported_transaction_version` - If the transaction version is not supported
/// * `unexpected_error` - If an unexpected error occurs

/// Following tests runs using V1 Invoke Transaction (params follow starknet-rs implementation)
#[rstest]
#[tokio::test]
async fn fail_if_param_(deoxys: JsonRpcClient<HttpTransport>) {
    let invalid_invoke_transaction = BroadcastedInvokeTransaction {
        sender_address: FieldElement::from_hex_be("valid_address").unwrap(),
        calldata: vec![FieldElement::from_hex_be("calldata_array").unwrap()],
        max_fee: FieldElement::from_hex_be("0x0ffffffff").unwrap(),
        signature: vec![FieldElement::from_hex_be("signature_array").unwrap()],
        nonce: FieldElement::from_hex_be("0x000000").unwrap(), //here nonce is invalid
        is_query: false,
    };

    let response_deoxys = deoxys
        .add_invoke_transaction(invalid_invoke_transaction)
        .await;

    assert_matches!(
        response_deoxys,
        Err(ProviderError::StarknetError(
            StarknetError::InvalidTransactionNonce
        ))
    );
}

#[rstest]
#[tokio::test]
async fn fail_if_insufficient_max_fee(deoxys: JsonRpcClient<HttpTransport>) {
    let invalid_invoke_transaction = BroadcastedInvokeTransaction {
        sender_address: FieldElement::from_hex_be("valid_address").unwrap(),
        calldata: vec![FieldElement::from_hex_be("calldata_array").unwrap()],
        max_fee: FieldElement::from_hex_be("0x000000").unwrap(), //here max_fee is insufficient
        signature: vec![FieldElement::from_hex_be("signature_array").unwrap()],
        nonce: FieldElement::from_hex_be("0x01").unwrap(),
        is_query: false,
    };

    let response_deoxys = deoxys
        .add_invoke_transaction(invalid_invoke_transaction)
        .await;

    assert_matches!(
        response_deoxys,
        Err(ProviderError::StarknetError(
            StarknetError::InsufficientMaxFee
        ))
    );
}

#[rstest]
#[tokio::test]
async fn fail_if_bad_calldata(deoxys: JsonRpcClient<HttpTransport>) {
    let invalid_invoke_transaction = BroadcastedInvokeTransaction {
        sender_address: FieldElement::from_hex_be("valid_address").unwrap(),
        calldata: vec![FieldElement::from_hex_be("0x000000").unwrap()], //here calldata is invalid
        max_fee: FieldElement::from_hex_be("0x0ffffffff").unwrap(),
        signature: vec![FieldElement::from_hex_be("signature_array").unwrap()],
        nonce: FieldElement::from_hex_be("0x01").unwrap(),
        is_query: false,
    };

    let response_deoxys = deoxys
        .add_invoke_transaction(invalid_invoke_transaction)
        .await;

    assert_matches!(
        response_deoxys,
        Err(ProviderError::StarknetError(
            StarknetError::ValidationFailure
        ))
    );
}

#[rstest]
#[tokio::test]
async fn works_ok_with_valid_params(deoxys: JsonRpcClient<HttpTransport>) {
    let valid_invoke_transaction = BroadcastedInvokeTransaction {
        sender_address: FieldElement::from_hex_be("valid_address").unwrap(),
        calldata: vec![FieldElement::from_hex_be("calldata_array").unwrap()],
        max_fee: FieldElement::from_hex_be("0x0ffffffff").unwrap(),
        signature: vec![FieldElement::from_hex_be("signature_array").unwrap()],
        nonce: FieldElement::from_hex_be("0x01").unwrap(),
        is_query: false,
    };

    //Here we added a valid transaction
    let response_deoxys = deoxys
        .add_invoke_transaction(valid_invoke_transaction)
        .await;

    //Now, if the transaction is valid, the rpc call response contain the transaction hash
    let transaction_submitted_hash = response_deoxys
        .expect("Transaction submition failed")
        .transaction_hash;

    //Wait for the transaction to be added to the chain
    thread::sleep(Duration::from_secs(15));

    //Let's check the transaction status
    let transaction_status = deoxys
        .get_transaction_status(transaction_submitted_hash)
        .await;

    assert_matches!(transaction_status.unwrap(), TransactionStatus::Received);
}
