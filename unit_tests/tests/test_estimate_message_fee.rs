#![feature(assert_matches)]

mod common;
use common::*;

use starknet_core::types::{BlockId, BlockTag, EthAddress, FieldElement, MsgFromL1, StarknetError};
use starknet_providers::{
    jsonrpc::{HttpTransport, JsonRpcClient},
    Provider, ProviderError,
};
use std::assert_matches::assert_matches;
use std::collections::HashMap;
use reqwest::Client;
use serde_json::json;
use serde::{Deserialize, Serialize};
use std::convert::From;
use serde_json::Error as SerdeError;
use reqwest::Error as ReqwestError;
use std::fmt;

/// Test for the `get_state_update` Deoxys RPC method
/// # Arguments
// * `message` - A message from the L1 containg a from address (Ethereum address), a to address (Starknet address),
//                      an entry point selector (hex string) and a payload (list of field elements)
// * `block_id` - The block id to get the state update from
//
// # Returns
// * `fee estimation` - The fee estimation for the given message
//
// # Errors
// * `block_not_found` - If the block is not found or invalid
// * `contract_not_found` - If the contract is not found or invalid
// * `contract_error` - If the contract is found but the message is invalid

/// Following tests runs with various example of messages from L1, 
/// you can of course modify address and payload to test with your own message.

pub fn get_message_from_l1(
    from: &str,
    to: FieldElement,
    selector: &str,
    load: &Vec<FieldElement>,
) -> MsgFromL1 {
    let loading = load.to_owned();
    MsgFromL1 {
        from_address: EthAddress::from_hex(from).unwrap(),
        to_address: to,
        entry_point_selector: FieldElement::from_hex_be(selector).unwrap(),
        payload: loading,
    }
}

#[rstest]
#[tokio::test]
async fn fail_non_existing_block(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[PATHFINDER];

    let payload_message: Vec<FieldElement> = vec![];
    let contract_address = FieldElement::from_hex_be("0x049D36570D4e46f48e99674bd3fcc84644DdD6b96F7C741B1562B82f9e004dC7").unwrap();
    let message = get_message_from_l1(
        "0xae0ee0a63a2ce6baeeffe56e7714fb4efe48d419", //ETH address
        contract_address,
        "0x000000",
        &payload_message,
    );

    let deoxys_message_fee = deoxys
        .estimate_message_fee(message, BlockId::Hash(FieldElement::ZERO))
        .await;
    assert_matches!(
        deoxys_message_fee,
        Err(ProviderError::StarknetError(StarknetError::BlockNotFound))
    );
}

#[rstest]
#[tokio::test]
async fn fail_contract_not_found(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[PATHFINDER];

    let unknown_contract_address =
        FieldElement::from_hex_be("0x4269DEADBEEF").expect("Invalid Contract Address");
    let payload_message: Vec<FieldElement> = vec![];
    let message = get_message_from_l1(
        "0xae0ee0a63a2ce6baeeffe56e7714fb4efe48d419",
        unknown_contract_address,
        "0x000000",
        &payload_message,
    );

    let deoxys_message_fee = deoxys
        .estimate_message_fee(message, BlockId::Tag(BlockTag::Latest))
        .await;
    assert_matches!(
        deoxys_message_fee,
        Err(ProviderError::StarknetError(
            StarknetError::ContractNotFound
        ))
    )
}

#[rstest]
#[tokio::test]
async fn fail_contract_error(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[PATHFINDER];

    //On this test, the contract address must be valid,
    //but the from_address, entry_point_selector or the payload must be invalid
    let payload_message: Vec<FieldElement> = vec![];
    let contract_address = FieldElement::from_hex_be("0x049D36570D4e46f48e99674bd3fcc84644DdD6b96F7C741B1562B82f9e004dC7").unwrap();
    let message = get_message_from_l1(
        "0xae0ee0a63a2ce6baeeffe56e7714fb4efe48d419",
        contract_address,
        "0x0000000",
        &payload_message,
    );

    let deoxys_message_fee = deoxys
        .estimate_message_fee(message, BlockId::Tag(BlockTag::Latest))
        .await;
    assert_matches!(
        deoxys_message_fee,
        Err(ProviderError::StarknetError(StarknetError::ContractError(
            _
        )))
    )
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse {
    jsonrpc: String,
    result: GasEstimate,
    id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct GasEstimate {
    gas_consumed: String,
    gas_price: String,
    overall_fee: String,
}

#[derive(Debug)]
enum CallError {
    HttpRequestError(ReqwestError),
    JsonParseError(SerdeError),
}

impl fmt::Display for CallError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CallError::HttpRequestError(ref err) => write!(f, "HTTP Request Error: {}", err),
            CallError::JsonParseError(ref err) => write!(f, "JSON Parse Error: {}", err),
        }
    }
}

impl From<ReqwestError> for CallError {
    fn from(error: ReqwestError) -> Self {
        CallError::HttpRequestError(error)
    }
}

impl From<SerdeError> for CallError {
    fn from(error: SerdeError) -> Self {
        CallError::JsonParseError(error)
    }
}

#[rstest]
#[tokio::test]
async fn estimate_message_fee_works_ok() -> Result<(), CallError> {
    let client = Client::new();
    let url = ""; // enter a valid RPC Endpoint here

    let request_body = json!({
        "id": 1,
        "jsonrpc": "2.0",
        "method": "starknet_estimateMessageFee",
        "params": [
            {
                "from_address": "0xae0ee0a63a2ce6baeeffe56e7714fb4efe48d419",
                "to_address": "0x073314940630fd6dcda0d772d4c972c4e0a9946bef9dabf4ef84eda8ef542b82",
                "entry_point_selector": "0x2d757788a8d8d6f21d1cd40bce38a8222d70654214e96ff95d8086e684fbee5",
                "payload": ["0x0622fc5c49c6b6b542e98b03b999f26f68b311512d6937c247a5a6a153df9a8a", "0x038d7ea4c68000", "0x00"]
            },
            {
                "block_number": 487440
            }
        ]
    });

    let response = client.post(url)
        .header("accept", "application/json")
        .header("content-type", "application/json")
        .json(&request_body)
        .send()
        .await?;

    let body = response.text().await?;
    let parsed_response: ApiResponse = serde_json::from_str(&body).map_err(CallError::from)?;
    
    assert_eq!(&parsed_response.result.gas_consumed, "0x4bd0", "Unexpected value for Gas Consumed");
    assert_eq!(&parsed_response.result.gas_price, "0x34b9d74ac", "Unexpected value for Gas Price");
    assert_eq!(&parsed_response.result.overall_fee, "0xf9d4911d2fc0", "Unexpected value for Overall Fee");

    Ok(())
}
