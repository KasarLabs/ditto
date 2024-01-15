#![feature(assert_matches)]

mod common;
use std::{assert_matches::assert_matches, collections::HashMap};

use common::*;
use starknet_core::types::{
    BlockId, BlockTag, FieldElement, StarknetError, TransactionExecutionStatus, TransactionStatus,
};
use starknet_providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider, ProviderError};

///
/// Unit test for `starknet_getTransactionStatus`
///
/// purpose: call getTransactionStatus on non-existent transaction hash.
/// fail case: non-existent transaction hash.
///
#[require(spec_version = "0.5.1")]
#[rstest]
#[tokio::test]
async fn fail_invalid_transaction(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];

    let response_deoxys = deoxys
        .get_transaction_status(FieldElement::ZERO)
        .await
        .err();

    assert_matches!(
        response_deoxys,
        Some(ProviderError::StarknetError(
            StarknetError::TransactionHashNotFound
        ))
    );
}

///
/// Unit test for `starknet_getTransactionStatus`
///
/// purpose: call getTransactionStatus on transaction which has been accepted on L1.
/// success case: retrieved transaction has been accepted on L1.
///
#[require(block_min = 50_000, spec_version = "0.5.1")]
#[rstest]
#[tokio::test]
async fn work_transaction_accepted_on_l1(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let response_deoxys = deoxys
        .get_transaction_status(FieldElement::from_hex_be(TRANSACTION_INVOKE).unwrap())
        .await
        .expect(ERR_DEOXYS);

    let response_pathfinder = pathfinder
        .get_transaction_status(FieldElement::from_hex_be(TRANSACTION_INVOKE).unwrap())
        .await
        .expect(ERR_PATHFINDER);

    assert_matches!(response_deoxys, TransactionStatus::AcceptedOnL1(_));
    assert_eq!(response_deoxys, response_pathfinder);
}

///
/// Unit test for `starknet_getTransactionStatus`
///
/// purpose: call getTransactionStatus on last transaction from the latest block.
/// success case: transaction is marked as accepted on L2.
///
#[require(block_min = "latest", spec_version = "0.5.1")]
#[rstest]
#[tokio::test]
async fn work_transaction_accepted_on_l2(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let transaction_count = deoxys
        .get_block_transaction_count(BlockId::Tag(BlockTag::Latest))
        .await
        .expect(ERR_DEOXYS);

    // last transaction of latest block
    let transaction = deoxys
        .get_transaction_by_block_id_and_index(
            BlockId::Tag(BlockTag::Latest),
            transaction_count - 1,
        )
        .await
        .expect(ERR_DEOXYS);

    let response_deoxys = deoxys
        .get_transaction_status(transaction.transaction_hash())
        .await
        .expect(ERR_DEOXYS);

    let response_pathfinder = pathfinder
        .get_transaction_status(transaction.transaction_hash())
        .await
        .expect(ERR_PATHFINDER);

    // note that transaction is still accepted on L2 if it is reverted!
    assert_matches!(response_deoxys, TransactionStatus::AcceptedOnL2(_));
    assert_eq!(response_deoxys, response_pathfinder);
}

///
/// Unit test for `starknet_getTransactionStatus`
///
/// purpose: call getTransactionStatus on reverted transaction.
/// success case: transaction is marked as reverted on L1.
///
#[require(block_min = 50_000, spec_version = "0.5.1")]
#[rstest]
#[tokio::test]
async fn work_transaction_reverted(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let response_deoxys = deoxys
        .get_transaction_status(FieldElement::from_hex_be(TRANSACTION_REVERTED).unwrap())
        .await
        .expect(ERR_DEOXYS);

    let response_pathfinder = pathfinder
        .get_transaction_status(FieldElement::from_hex_be(TRANSACTION_REVERTED).unwrap())
        .await
        .expect(ERR_PATHFINDER);

    assert_matches!(
        response_deoxys,
        TransactionStatus::AcceptedOnL1(TransactionExecutionStatus::Reverted)
    );
    assert_eq!(response_deoxys, response_pathfinder);
}
