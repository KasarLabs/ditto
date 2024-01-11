#![feature(assert_matches)]

mod common;
use std::sync::Arc;

use anyhow::anyhow;
use common::*;
use starknet::macros::{selector, felt_hex};
use starknet_core::types::{EventFilter, BlockId, FieldElement};
use starknet_providers::{JsonRpcClient, jsonrpc::HttpTransport, Provider};
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

// event type to retrieve
const EVENT_KEY: FieldElement = selector!("transaction_executed");
const BLOCK_NU: u64 = 50000;
const BLOCK_HASH: FieldElement = felt_hex!("0x053315a56543737cd1b2dc40c60e84d03a9b10d712c9b29f488dc979f0cd56bd");
const BLOCK_RANGE: u64 = 100;

///
/// Unit test for `starknet_getEvents`
/// 
/// purpose: call getEvents on a valid block with a single selector.
/// success case: valid events format, events point to valid transactions.
/// 
#[rstest]
#[tokio::test]
#[logging]
async fn work_valid_call_single_selector(deoxys: JsonRpcClient<HttpTransport>, pathfinder: JsonRpcClient<HttpTransport>) {
    let response_deoxys = deoxys.get_events(
        EventFilter {
            // getEvents is a applied through a filter
            // this filter consists of a block range...
            from_block: Some(BlockId::Number(BLOCK_NU)),
            to_block: Some(BlockId::Number(BLOCK_NU)),
            // a beginning contract address...
            address: None,
            // and keys used to filter out events. Keys can include a hash of the event 
            // and even event return values for further filtering
            keys: Some(vec![vec![EVENT_KEY]])
        },
        // in cases were a first search does not yield enough results, a continuation key 
        // can be used to keep searching from the point of the last getEvent search
        None, 
        // chunk size marks the number of events to look through and filter
        // this means that there cannot be more than chunk_size events returned by getEvents
        BLOCK_RANGE
    ).await.expect(ERR_DEOXYS);

    let response_pathfinder = pathfinder.get_events(
        EventFilter {
            from_block: Some(BlockId::Number(BLOCK_NU)),
            to_block: Some(BlockId::Number(BLOCK_NU)),
            address: None,
            keys: Some(vec![vec![EVENT_KEY]])
        },
        None, 
        100
    ).await.expect(ERR_PATHFINDER);

    // surface-level comparison with pathfinder node, not a guarantee of structure
    assert_eq!(response_deoxys, response_pathfinder);
    log::info!("Events at block {BLOCK_NU}: {}", serde_json::to_string_pretty(&response_deoxys).unwrap());

    let mut task_set = JoinSet::new();
    let arc_deoxys = Arc::new(deoxys);

    // detailed comparisons are done in parallel as they are computationally expensive
    for event in response_deoxys.events {
        let arc_event = Arc::new(event);

        // TODO: simple comparisons are CPU bound, consider running this is Rayon
        let clone_event = Arc::clone(&arc_event);
        task_set.spawn(async move {
            assert_eq!(clone_event.keys.len(), 1);
            // first key is always event key
            // further keys cannot be predicted without knowing event contract
            assert_eq!(clone_event.keys.first().unwrap(), &EVENT_KEY);
            assert_eq!(clone_event.block_hash, BLOCK_HASH);
            assert_eq!(clone_event.block_number, BLOCK_NU);
            
            assert_ne!(clone_event.data.len(), 0);
            // first data is always transaction hash, 
            // further data cannot be predicted without knowing event contract
            assert_eq!(clone_event.data.first().unwrap(), &clone_event.transaction_hash);

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
            match clone_deoxys.get_transaction_by_hash(clone_event.transaction_hash).await {
                Ok(_) => anyhow::Ok(()),
                Err(e) => Err(anyhow!("{e}: transaction 0x{:064x} does not exist", clone_event.transaction_hash)),
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
            },
            // thread execution error, stop all threads (invalid assertion or tokio error)
            Err(e) => {
                task_set.abort_all();
                panic!("JoinError: {}", e);
            },
        }
    }
}