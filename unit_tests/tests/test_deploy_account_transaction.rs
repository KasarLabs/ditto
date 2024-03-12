#![feature(assert_matches)]

mod common;
use common::*;
use starknet_core::types::{
    BroadcastedDeployAccountTransaction, FieldElement, StarknetError, TransactionStatus,
};
use starknet_providers::{
    jsonrpc::{HttpTransport, JsonRpcClient},
    Provider, ProviderError,
};
use std::assert_matches::assert_matches;
use std::thread;
use std::time::Duration;

/// Test for the `deploy_account_transaction` Deoxys RPC method
/// Submit a new deploy account transaction
///
/// There is two type of DeployAccountTransaction: V1 and V3
///
/// # Arguments
/// * `deploy_account_transaction` - A deploy account transaction
///    with following fields (V1):
///       * `type` - DEPLOY_ACCOUNT
///       * `max_fee` - The maximal fee willing to be paid
///       * `signature` - The transaction signature
///       * `nonce` - The nonce of the transaction
///       * `contract_address_salt` - The salt for the address of the deployed contract
///       * `constructor_calldata` - The parameters passed to the constructor
///       * `class_hash` - The hash of the deployed contract's class
///       * `is_query` - If set to `true`, uses a query-only transaction version that's invalid for execution
///
/// # Returns
/// * `result` - The result of the transaction submission
///    with following fields:
///       * `transaction_hash` - The hash of the transaction
///       * `contract_address` - The address of the deployed contract
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

#[ignore = "For this one, you need to submit a valid account (private key) and address"]
#[rstest]
#[tokio::test]
async fn fail_if_param_(deoxys: JsonRpcClient<HttpTransport>) {
    let invalid_deploy_account_transaction = BroadcastedDeployAccountTransaction {
        max_fee: FieldElement::from_hex_be("0x0ffffffff").unwrap(),
        signature: vec![FieldElement::from_hex_be("signature_array").unwrap()],
        nonce: FieldElement::from_hex_be("0x000000").unwrap(), //here nonce is invalid
        contract_address_salt: FieldElement::from_hex_be("0x000000").unwrap(),
        constructor_calldata: vec![FieldElement::from_hex_be("constructor_calldata_array").unwrap()],
        class_hash: FieldElement::from_hex_be("0x000000").unwrap(),
        is_query: false,
    };

    let response_deoxys = deoxys
        .add_deploy_account_transaction(invalid_deploy_account_transaction)
        .await;

    assert_matches!(
        response_deoxys,
        Err(ProviderError::StarknetError(
            StarknetError::InvalidTransactionNonce
        ))
    );
}

#[ignore = "For this one, you need to submit a valid account (private key) and address"]
#[rstest]
#[tokio::test]
async fn fail_if_insufficient_max_fee(deoxys: JsonRpcClient<HttpTransport>) {
    let invalid_deploy_account_transaction = BroadcastedDeployAccountTransaction {
        max_fee: FieldElement::from_hex_be("0x000000").unwrap(), //here max_fee is insufficient
        signature: vec![FieldElement::from_hex_be("signature_array").unwrap()],
        nonce: FieldElement::from_hex_be("0x000000").unwrap(),
        contract_address_salt: FieldElement::from_hex_be("0x000000").unwrap(),
        constructor_calldata: vec![FieldElement::from_hex_be("constructor_calldata_array").unwrap()],
        class_hash: FieldElement::from_hex_be("0x000000").unwrap(),
        is_query: false,
    };

    let response_deoxys = deoxys
        .add_deploy_account_transaction(invalid_deploy_account_transaction)
        .await;

    assert_matches!(
        response_deoxys,
        Err(ProviderError::StarknetError(
            StarknetError::InsufficientMaxFee
        ))
    );
}

#[ignore = "For this one, you need to submit a valid account (private key) and address"]
#[rstest]
#[tokio::test]
async fn fail_if_invalid_transaction_nonce(deoxys: JsonRpcClient<HttpTransport>) {
    let invalid_deploy_account_transaction = BroadcastedDeployAccountTransaction {
        max_fee: FieldElement::from_hex_be("0x0ffffffff").unwrap(),
        signature: vec![FieldElement::from_hex_be("signature_array").unwrap()],
        nonce: FieldElement::from_hex_be("0x000000").unwrap(), //here nonce is invalid
        contract_address_salt: FieldElement::from_hex_be("0x000000").unwrap(),
        constructor_calldata: vec![FieldElement::from_hex_be("constructor_calldata_array").unwrap()],
        class_hash: FieldElement::from_hex_be("0x000000").unwrap(),
        is_query: false,
    };

    let response_deoxys = deoxys
        .add_deploy_account_transaction(invalid_deploy_account_transaction)
        .await;

    assert_matches!(
        response_deoxys,
        Err(ProviderError::StarknetError(
            StarknetError::InvalidTransactionNonce
        ))
    );
}

#[ignore = "For this one, you need to submit a valid account (private key) and address"]
#[rstest]
#[tokio::test]
async fn works_ok(deoxys: JsonRpcClient<HttpTransport>) {
    let valid_deploy_account_transaction = BroadcastedDeployAccountTransaction {
        max_fee: FieldElement::from_hex_be("0x0ffffffff").unwrap(),
        signature: vec![FieldElement::from_hex_be("signature_array").unwrap()],
        nonce: FieldElement::from_hex_be("0x000000").unwrap(),
        contract_address_salt: FieldElement::from_hex_be("0x000000").unwrap(),
        constructor_calldata: vec![FieldElement::from_hex_be("constructor_calldata_array").unwrap()],
        class_hash: FieldElement::from_hex_be("0x000000").unwrap(),
        is_query: false,
    };

    let response_deoxys = deoxys
        .add_deploy_account_transaction(valid_deploy_account_transaction)
        .await;

    //Here, as response we got the transaction hash and the contract address deployed
    let result = response_deoxys.expect("Error in the transaction submission");

    //Now, if the transaction is valid, the rpc call response contain the transaction hash
    let transaction_submitted_hash = result.transaction_hash;

    //Wait for the transaction to be added to the chain
    thread::sleep(Duration::from_secs(15));

    //Let's check the transaction status
    let transaction_status = deoxys
        .get_transaction_status(transaction_submitted_hash)
        .await;

    assert_matches!(transaction_status.unwrap(), TransactionStatus::Received);
}
