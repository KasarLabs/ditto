#![feature(assert_matches)]

mod common;
use std::{assert_matches::assert_matches, collections::HashMap};

use common::*;
use starknet_core::types::{
    BlockId, BlockTag, FieldElement, StarknetError, TransactionExecutionStatus, TransactionStatus,
};
use starknet_providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider};

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

    let response_deoxys = deoxys.get_transaction_status(FieldElement::ZERO).await;

    assert!(
        response_deoxys.is_err(),
        "Expected an error, but got a result"
    );

    if let Err(error) = response_deoxys {
        let is_correct_error =
            checking_error_format(&error, StarknetError::TransactionHashNotFound);

        assert!(
            is_correct_error,
            "Expected TransactionHashNotFound error, but got a different error"
        );
    }
}

///
/// Unit test for `starknet_getTransactionStatus`
///
/// purpose: call getTransactionStatus on transaction which has been accepted on L1.
/// success case: retrieved transaction has been accepted on L1.
///
#[rstest]
#[tokio::test]
async fn work_transaction_accepted_on_l1(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    if MAX_BLOCK < 5001 {
        return;
    }
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
#[rstest]
#[tokio::test]
#[ignore = "slash this ignore when Deoxys node is fully synced"]
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
#[rstest]
#[tokio::test]
async fn work_transaction_reverted(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];
 
    if MAX_BLOCK < 500672 {
        return;
    }
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

async fn work_with_hash(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
    transaction_hash: &str,
) {
    let tx = FieldElement::from_hex_be(transaction_hash).unwrap();
    let response_deoxys = deoxys.get_transaction_status(tx).await.expect(ERR_DEOXYS);

    let response_pathfinder = pathfinder
        .get_transaction_status(tx)
        .await
        .expect(ERR_PATHFINDER);

    assert_eq!(response_deoxys, response_pathfinder);
}

/// first transaction on block 0
#[rstest]
#[tokio::test]
async fn work_with_first_transaction_block_0(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
) {
    work_with_hash(
        deoxys,
        pathfinder,
        "0xe0a2e45a80bb827967e096bcf58874f6c01c191e0a0530624cba66a508ae75",
    )
    .await;
}

/// deploy transaction on block 0
#[rstest]
#[tokio::test]
async fn work_with_deploy_transaction_block_0(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
) {
    work_with_hash(
        deoxys,
        pathfinder,
        "0x12c96ae3c050771689eb261c9bf78fac2580708c7f1f3d69a9647d8be59f1e1",
    )
    .await;
}

///invoke transaction on block 0
#[rstest]
#[tokio::test]
async fn work_with_invoke_transaction_block_0(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
) {
    work_with_hash(
        deoxys,
        pathfinder,
        "0xce54bbc5647e1c1ea4276c01a708523f740db0ff5474c77734f73beec2624",
    )
    .await;
}

///deploy transaction on block 1
#[rstest]
#[tokio::test]
async fn work_with_deploy_transaction_block_1(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
) {
    work_with_hash(
        deoxys,
        pathfinder,
        "0x2f07a65f9f7a6445b2a0b1fb90ef12f5fd3b94128d06a67712efd3b2f163533",
    )
    .await;
}
