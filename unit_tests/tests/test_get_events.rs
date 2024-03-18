#![feature(assert_matches)]

mod common;
use std::sync::Arc;

use anyhow::anyhow;
use common::*;
use starknet::macros::{felt_hex, selector};
use starknet_core::types::{BlockId, EventFilter, EventsPage, FieldElement};
use starknet_providers::{
    jsonrpc::{HttpTransport, JsonRpcError},
    JsonRpcClient, Provider, ProviderError,
};
use tokio::task::JoinSet;

///
/// Test for RPC call `starknet_getEvents`.
///
/// *What is an EVENT*
///
/// An Event in the context of the Starknet blockchain is called a Cairo event. It can be emitted by smart
/// contract as a side effect marking steps in execution or internal state. These in turn can be observed
/// from the outside and reacted upon. Events are marked by a *key* and can also store *data.
///
/// Ex: an authentication contract might emit an event upon successfully login in a user, or an error event
/// in case of invalid data.
///
/// More documentation can be found in [the Starknet Book](https://www.starknetjs.com/docs/guides/events/)
///

///
/// Unit test for `starknet_getEvents`
///
/// purpose: call getEvents on an invalid block number.
/// fail case: invalid block number (invalid param).
///
#[rstest]
#[tokio::test]
#[logging]
async fn fail_invalid_block_number(deoxys: JsonRpcClient<HttpTransport>) {
    let keys: Vec<Vec<FieldElement>> = vec![vec![selector!("transaction_executed")]];
    let block_nu: u64 = u64::MAX;
    let block_range: u64 = 100;

    let response_deoxys = get_events(&deoxys, &keys, block_nu, block_range).await;

    assert!(
        response_deoxys.is_err(),
        "Expected an error, but got a result"
    );

    let expected_error = JsonRpcError {
        code: -32602,
        message: "Invalid params".to_string(),
        data: None,
    };

    assert!(
        response_deoxys.is_err(),
        "Expected an error response, but got result. Expected error: {:?}",
        expected_error
    );
}

///
/// Unit test for `starknet_getEvents`
///
/// purpose: call getEvents on an invalid event selector.
/// fail case: invalid event selector.
///
#[rstest]
#[tokio::test]
#[logging]
async fn fail_invalid_keys(deoxys: JsonRpcClient<HttpTransport>) {
    let keys: Vec<Vec<FieldElement>> = vec![vec![selector!("")]];
    let block_nu: u64 = 50000;
    let block_range: u64 = 100;

    let response_deoxys = get_events(&deoxys, &keys, block_nu, block_range)
        .await
        .expect(ERR_DEOXYS);

    log::info!(
        "Events at block {block_nu}: {}",
        serde_json::to_string_pretty(&response_deoxys).unwrap()
    );
    assert_eq!(response_deoxys.events.len(), 0);
}

///
/// Unit test for `starknet_getEvents`
///
/// purpose: call getEvents on an invalid event selector.
/// fail case: invalid event selector.
///
#[rstest]
#[tokio::test]
#[logging]
async fn fail_invalid_block_range(deoxys: JsonRpcClient<HttpTransport>) {
    let keys: Vec<Vec<FieldElement>> = vec![vec![selector!("")]];
    let block_nu: u64 = 50000;
    let block_range: u64 = 0;

    let response_deoxys = get_events(&deoxys, &keys, block_nu, block_range).await;

    // for some reason a block range of 0 results in an internal error
    assert!(
        response_deoxys.is_err(),
        "Expected an error, but got a result"
    );

    let expected_error = JsonRpcError {
        code: -32602,
        message: "requested page size is too small, supported minimum is 1".to_string(),
        data: None,
    };

    assert!(
        response_deoxys.is_err(),
        "Expected an error response, but got result. Expected error: {:?}",
        expected_error
    );
}

///
/// Unit test for `starknet_getEvents`
///
/// purpose: call getEvents on a valid block with a no selector.
/// success case: retrieves the first 100 events of that block.
///
#[rstest]
#[tokio::test]
#[logging]
async fn work_valid_call_no_selector(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
) {
    let keys: Vec<Vec<FieldElement>> = vec![vec![selector!("transaction_executed")]];
    let block_nu: u64 = 50000;
    let block_hash: FieldElement =
        felt_hex!("0x053315a56543737cd1b2dc40c60e84d03a9b10d712c9b29f488dc979f0cd56bd");
    let block_range: u64 = 100;

    let response_deoxys = get_events(&deoxys, &keys, block_nu, block_range)
        .await
        .expect(ERR_DEOXYS);
    let response_pathfinder = get_events(&pathfinder, &keys, block_nu, block_range)
        .await
        .expect(ERR_PATHFINDER);

    log::info!(
        "Events at block {block_nu}: {}",
        serde_json::to_string_pretty(&response_deoxys).unwrap()
    );
    assert_eq!(response_deoxys.events.len(), block_range as usize);
    assert_eq!(response_deoxys, response_pathfinder);
    deep_check_events(deoxys, response_deoxys, keys, block_hash, block_nu).await;
}

///
/// Unit test for `starknet_getEvents`
///
/// purpose: call getEvents on a valid block with a single selector.
/// success case: valid events format, events point to valid transactions.
///
#[rstest]
#[tokio::test]
#[logging]
async fn work_valid_call_single_selector(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
) {
    // event type to retrieve
    let keys: Vec<Vec<FieldElement>> = vec![vec![selector!("transaction_executed")]];
    let block_nu: u64 = 50000;
    let block_hash: FieldElement =
        felt_hex!("0x053315a56543737cd1b2dc40c60e84d03a9b10d712c9b29f488dc979f0cd56bd");
    let block_range: u64 = 100;

    let response_deoxys = get_events(&deoxys, &keys, block_nu, block_range)
        .await
        .expect(ERR_DEOXYS);
    let response_pathfinder = get_events(&pathfinder, &keys, block_nu, block_range)
        .await
        .expect(ERR_PATHFINDER);

    log::info!(
        "Events at block {block_nu}: {}",
        serde_json::to_string_pretty(&response_deoxys).unwrap()
    );
    assert_eq!(response_deoxys, response_pathfinder);
    deep_check_events(deoxys, response_deoxys, keys, block_hash, block_nu).await;
}

///
/// Unit test for `starknet_getEvents`
///
/// purpose: call getEvents on a valid block with a multiple selectors.
/// success case: retrieves all events matching the selector in the first 100 events of that block
///               + valid event format and valid transactions.
///
#[rstest]
#[tokio::test]
#[logging]
async fn work_valid_call_multiple_selector(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
) {
    let keys: Vec<Vec<FieldElement>> = vec![vec![
        selector!("transaction_executed"),
        selector!("account_created"),
    ]];
    let block_nu: u64 = 50000;
    let block_hash: FieldElement =
        felt_hex!("0x053315a56543737cd1b2dc40c60e84d03a9b10d712c9b29f488dc979f0cd56bd");
    let block_range: u64 = 100;

    let response_deoxys = get_events(&deoxys, &keys, block_nu, block_range)
        .await
        .expect(ERR_DEOXYS);
    let response_pathfinder = get_events(&pathfinder, &keys, block_nu, block_range)
        .await
        .expect(ERR_PATHFINDER);

    log::info!(
        "Events at block {block_nu}: {}",
        serde_json::to_string_pretty(&response_deoxys).unwrap()
    );
    assert_eq!(response_deoxys, response_pathfinder);
    deep_check_events(deoxys, response_deoxys, keys, block_hash, block_nu).await;
}

async fn get_events(
    client: &JsonRpcClient<HttpTransport>,
    keys: &[Vec<FieldElement>],
    block_nu: u64,
    block_range: u64,
) -> Result<EventsPage, ProviderError> {
    client
        .get_events(
            EventFilter {
                // getEvents is a applied through a filter
                // this filter consists of a block range...
                from_block: Some(BlockId::Number(block_nu)),
                to_block: Some(BlockId::Number(block_nu)),
                // a beginning contract address...
                address: None,
                // and keys used to filter out events. Keys can include a hash of the event
                // and even event return values for further filtering
                keys: Some(keys.to_vec()),
            },
            // in cases were a first search does not yield enough results, a continuation key
            // can be used to keep searching from the point of the last getEvent search
            None,
            // chunk size marks the number of events to look through and filter
            // this means that there cannot be more than chunk_size events returned by getEvents
            block_range,
        )
        .await
}

async fn deep_check_events(
    deoxys: JsonRpcClient<HttpTransport>,
    response_deoxys: EventsPage,
    keys: Vec<Vec<FieldElement>>,
    block_hash: FieldElement,
    block_nu: u64,
) {
    let mut task_set = JoinSet::new();
    let arc_deoxys = Arc::new(deoxys);
    let arc_keys = Arc::new(keys);

    // detailed comparisons are done in parallel as they are computationally expensive
    for event in response_deoxys.events {
        let arc_event = Arc::new(event);

        // TODO: simple comparisons are CPU bound, consider running this is Rayon
        let clone_event = Arc::clone(&arc_event);
        let clone_keys = Arc::clone(&arc_keys);

        task_set.spawn(async move {
            assert_eq!(clone_event.keys.len(), 1);
            // first key is always event key
            // further keys cannot be predicted without knowing event contract
            assert!(clone_keys
                .first()
                .unwrap()
                .contains(clone_event.keys.first().unwrap()));
            assert_eq!(clone_event.block_hash, block_hash);
            assert_eq!(clone_event.block_number, block_nu);
            assert_ne!(clone_event.data.len(), 0);

            anyhow::Ok(())
        });

        // to make sure the associated transactions retrieved by getEvents exists, we make
        // a call to the blockchain to retrieve them. This is an io-bound operation
        // and benefits greatly of parallelization through tokio
        //
        // Warning: this test is fast enough as to be rate-limited on certain nodes,
        // make sure you are running your own instance or using a provider with rate limiting disabled
        let clone_event = Arc::clone(&arc_event);
        let clone_deoxys = Arc::clone(&arc_deoxys);
        task_set.spawn(async move {
            match clone_deoxys
                .get_transaction_by_hash(clone_event.transaction_hash)
                .await
            {
                Ok(_) => anyhow::Ok(()),
                Err(e) => Err(anyhow!(
                    "{e}: transaction 0x{:064x} does not exist",
                    clone_event.transaction_hash
                )),
            }
        });
    }

    // waits for all tasks to complete. Errors occur if an assertion fails
    // or transaction cannot be found with getTransactionByHash
    while let Some(res) = task_set.join_next().await {
        match res {
            // invalid result, could not find transaction
            Ok(r) => {
                if let Err(e) = r {
                    task_set.abort_all();
                    panic!("{}", e);
                }
            }
            // thread execution error, stop all threads (invalid assertion or tokio error)
            Err(e) => {
                task_set.abort_all();
                panic!("JoinError: {}", e);
            }
        }
    }
}
