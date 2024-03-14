#![feature(assert_matches)]

mod common;
use common::*;
use starknet_core::types::{BlockId, BlockTag, EthAddress, FieldElement, MsgFromL1, StarknetError, ContractErrorData};
use starknet_providers::{
    jsonrpc::{HttpTransport, JsonRpcClient},
    Provider,
};

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
async fn fail_non_existing_block(deoxys: JsonRpcClient<HttpTransport>) {
    let payload_message: Vec<FieldElement> = vec![];
    let contract_address = FieldElement::from_hex_be(
        "0x049D36570D4e46f48e99674bd3fcc84644DdD6b96F7C741B1562B82f9e004dC7",
    )
    .unwrap();
    let message = get_message_from_l1(
        "0xae0ee0a63a2ce6baeeffe56e7714fb4efe48d419", //ETH address
        contract_address,
        "0x000000",
        &payload_message,
    );

    let response_deoxys = deoxys
        .estimate_message_fee(message, BlockId::Hash(FieldElement::ZERO))
        .await;

    assert!(
        response_deoxys.is_err(),
        "Expected an error, but got a result"
    );

    if let Err(error) = response_deoxys {
        let is_correct_error = checking_error_format(
            &error,
            StarknetError::BlockNotFound,
        );

        assert!(
            is_correct_error,
            "Expected BlockNotFound error, but got a different error"
        );
    }
}

// Care, Juno and Pathfinder error differ on this one
#[rstest]
#[tokio::test]
async fn fail_contract_not_found(deoxys: JsonRpcClient<HttpTransport>) {
    let unknown_contract_address =
        FieldElement::from_hex_be("0x4269DEADBEEF").expect("Invalid Contract Address");
    let payload_message: Vec<FieldElement> = vec![];
    let message = get_message_from_l1(
        "0xae0ee0a63a2ce6baeeffe56e7714fb4efe48d419",
        unknown_contract_address,
        "0x000000",
        &payload_message,
    );

    let response_deoxys = deoxys
        .estimate_message_fee(message, BlockId::Tag(BlockTag::Latest))
        .await;

    println!("{:?}", response_deoxys);

    assert!(
        response_deoxys.is_err(),
        "Expected an error, but got a result"
    );

    let revert_error = ContractErrorData {
        revert_error: "Transaction execution has failed".to_string(),
    };

    if let Err(error) = response_deoxys {
        let is_contract_not_found = checking_error_format(
            &error,
            StarknetError::ContractNotFound,
        );

        let is_contract_error = checking_error_format(
            &error,
            StarknetError::ContractError(revert_error)
        );

        assert!(
            is_contract_not_found || is_contract_error,
            "Expected ContractNotFound or ContractError, but got a different error"
        );
    }
}


#[rstest]
#[tokio::test]
async fn fail_contract_error(deoxys: JsonRpcClient<HttpTransport>) {
    //On this test, the contract address must be valid,
    //but the from_address, entry_point_selector or the payload must be invalid
    let payload_message: Vec<FieldElement> = vec![];
    let contract_address = FieldElement::from_hex_be(
        "0x049D36570D4e46f48e99674bd3fcc84644DdD6b96F7C741B1562B82f9e004dC7",
    )
    .unwrap();
    let message = get_message_from_l1(
        "0xae0ee0a63a2ce6baeeffe56e7714fb4efe48d419",
        contract_address,
        "0x0000000",
        &payload_message,
    );

    let response_deoxys = deoxys
        .estimate_message_fee(message, BlockId::Tag(BlockTag::Latest))
        .await;

    let error_reason = ContractErrorData {
        revert_error: "ContractError".to_string(),
    };

    assert!(
        response_deoxys.is_err(),
        "Expected an error, but got a result"
    );

    if let Err(error) = response_deoxys {
        let is_correct_error = checking_error_format(
            &error,
            StarknetError::ContractError(error_reason),
        );

        assert!(
            is_correct_error,
            "Expected Contract error, but got a different error"
        );
    }
}

#[rstest]
#[tokio::test]
async fn estimate_message_fee_works_ok(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
) {
    let contract_address = FieldElement::from_hex_be(
        "0x073314940630fd6dcda0d772d4c972c4e0a9946bef9dabf4ef84eda8ef542b82",
    );
    let payload_hex_strings = vec![
        "0x0622fc5c49c6b6b542e98b03b999f26f68b311512d6937c247a5a6a153df9a8a",
        "0x038d7ea4c68000",
        "0x00",
    ];

    let payload_message: Vec<FieldElement> = payload_hex_strings
        .iter()
        .map(|hex_str| FieldElement::from_hex_be(hex_str).unwrap())
        .collect();

    let message_fee_params = get_message_from_l1(
        "0xae0ee0a63a2ce6baeeffe56e7714fb4efe48d419",
        contract_address.expect(""),
        "0x2d757788a8d8d6f21d1cd40bce38a8222d70654214e96ff95d8086e684fbee5",
        &payload_message,
    );

    let deoxys_message_fee = deoxys
        .estimate_message_fee(message_fee_params.clone(), BlockId::Tag(BlockTag::Latest))
        .await
        .unwrap();

    let pathfinder_message_fee = pathfinder
        .estimate_message_fee(message_fee_params, BlockId::Tag(BlockTag::Latest))
        .await
        .unwrap();

    assert_eq!(
        deoxys_message_fee.gas_consumed,
        pathfinder_message_fee.gas_consumed
    );
    assert_eq!(
        deoxys_message_fee.gas_price,
        pathfinder_message_fee.gas_price
    );
    assert_eq!(
        deoxys_message_fee.overall_fee,
        pathfinder_message_fee.overall_fee
    );
}
