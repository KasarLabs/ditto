#![feature(assert_matches)]

mod common;
use common::*;

use std::assert_matches::assert_matches;
use std::sync::Arc;

use starknet_core::types::{
    BlockId, BlockTag, FieldElement, NoTraceAvailableErrorData, SequencerTransactionStatus,
    StarknetError,
};
use starknet_providers::{
    jsonrpc::{HttpTransport, JsonRpcClient},
    Provider,
};

#[rstest]
#[tokio::test]
async fn fail_non_existing_hash(deoxys: JsonRpcClient<HttpTransport>) {
    let transaction_hash = FieldElement::from_hex_be(
        "0x04456c75586c033f4c8f6731a87d10ff5779e40c351e9c8378590ae2a3f823d1",
    )
    .unwrap(); // non-existent transaction hash

    let response_deoxys = deoxys.trace_transaction(transaction_hash).await;

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

/// Starknet-spec : Extra information on why trace is not available. Either it wasn't executed yet (RECEIVED), or the transaction failed (REJECTED.
#[rstest]
#[tokio::test]
async fn fail_no_trace_available(deoxys: JsonRpcClient<HttpTransport>) {
    let transaction_hash = FieldElement::from_hex_be(
        "0x2062dc37facfcc3bed03163dbbde0e3874bf8b231628c6aa21ac2d094b94372",
    )
    .unwrap(); // first tx reverted at block 164901

    let response_deoxys = deoxys.trace_transaction(transaction_hash).await;

    assert!(
        response_deoxys.is_err(),
        "Expected an error, but got a result"
    );

    if let Err(error) = response_deoxys {
        let is_correct_error = checking_error_format(
            &error,
            StarknetError::NoTraceAvailable(NoTraceAvailableErrorData {
                status: SequencerTransactionStatus::Rejected, //Check this because here Pathfinder and Juno return a ContractError but with a revert_reason":"Error in the called contract,
            }),
        );

        assert!(
            is_correct_error,
            "Expected NoTraceAvailable error, but got a different error"
        );
    }
}

#[rstest]
#[tokio::test]
async fn work_trace_transaction(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
) {
    let transaction_hash = FieldElement::from_hex_be(
        "0x04456c75586c033f4c8f6731a87d10ff5779e40c351e9c8378590ae2a3f823da",
    )
    .unwrap(); // first tx accepted at block 10000

    let response_deoxys = deoxys
        .trace_transaction(transaction_hash)
        .await
        .expect(ERR_DEOXYS);

    let response_pathfinder = pathfinder
        .trace_transaction(transaction_hash)
        .await
        .expect(ERR_PATHFINDER);

    assert_eq!(response_deoxys, response_pathfinder);
}
