#![feature(assert_matches)]

mod common;
use common::*;

use starknet_core::types::{
    BlockId, BlockTag, BroadcastedInvokeTransaction, BroadcastedInvokeTransactionV1,
    BroadcastedTransaction, ContractErrorData, FieldElement, SimulationFlag, StarknetError,
};
use starknet_core::utils::get_selector_from_name;
use starknet_providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider};
use std::convert::From;

/// Test for the `simulate transaction` Deoxys RPC Call
/// Simulate a given sequence of transactions on the requested state, and generate the execution traces.
/// Note that some of the transactions may revert, in which case no error is thrown, but revert details can be seen on the returned trace object.
/// Note that some of the transactions may revert, this will be reflected by the revert_error property in the trace.
///
/// # Arguments
// * `transactions` - A sequence of transactions to simulate, running each transaction on the state resulting from applying all the previous ones
// * `block_id` - The hash of the requested block, or number (height) of the requested block, or a block tag,
// * `simulation_flags` - Describes what parts of the transaction should be executed
//
// # Returns
// * `simulated_transactions` - The execution trace and consuemd resources of the required transactions
//
// # Errors
// * `block_not_found` - If the block is not found or invalid
// * `transaction_execution_error` - If one of the transactions failed to execute

/// 🚧 Care this method is tested on v_0.6

// pub fn get_broadcasted_transaction(
//     from: &str,
//     to: FieldElement,
//     selector: &str,
//     load: &Vec<FieldElement>,
// ) -> BroadcastedTransaction {
//     let loading = load.to_owned();
//     MsgFromL1 {
//         from_address: EthAddress::from_hex(from).unwrap(),
//         to_address: to,
//         entry_point_selector: FieldElement::from_hex_be(selector).unwrap(),
//         payload: loading,
//     }
// }

#[rstest]
#[tokio::test]
async fn fail_non_existing_block(deoxys: JsonRpcClient<HttpTransport>) {
    let ok_invoke_transaction = BroadcastedTransaction::Invoke(BroadcastedInvokeTransaction::V1(
        BroadcastedInvokeTransactionV1 {
            max_fee: FieldElement::ZERO,
            signature: vec![],
            nonce: FieldElement::ZERO,
            sender_address: FieldElement::from_hex_be(ACCOUNT_CONTRACT).unwrap(),
            calldata: vec![
                FieldElement::from_hex_be(TEST_CONTRACT_ADDRESS).unwrap(),
                get_selector_from_name("sqrt").unwrap(),
                FieldElement::from_hex_be("1").unwrap(),
                FieldElement::from(81u8),
            ],
            is_query: false,
        },
    ));

    let response_deoxys = deoxys
        .simulate_transactions(
            BlockId::Hash(FieldElement::ZERO),
            &[ok_invoke_transaction],
            [],
        )
        .await;

    assert!(
        response_deoxys.is_err(),
        "Expected an error, but got a result"
    );

    if let Err(error) = response_deoxys {
        let is_correct_error = checking_error_format(&error, StarknetError::BlockNotFound);

        assert!(
            is_correct_error,
            "Expected BlockNotFound error, but got a different error"
        );
    }
}

#[rstest]
#[tokio::test]
async fn fail_max_fee_too_big(deoxys: JsonRpcClient<HttpTransport>) {
    let max_fee_invoke_transaction = BroadcastedTransaction::Invoke(
        BroadcastedInvokeTransaction::V1(BroadcastedInvokeTransactionV1 {
            max_fee: FieldElement::from_hex_be("0xffffffffffffffffff").unwrap(),
            signature: vec![
                FieldElement::from_hex_be(
                    "0x5687164368262e1885f904c31bfe55362d91b9a5195d220d5d59aa3c8286349",
                )
                .expect("REASON"),
                FieldElement::from_hex_be(
                    "0x2bf8dd834492afe810152fe45083b8c768d62556f772885624ccbd52c6b80d7",
                )
                .expect("REASON"),
            ],
            nonce: FieldElement::from_hex_be("0x22").unwrap(),
            sender_address: FieldElement::from_hex_be(
                "0x019f57133d6a46990231a58a8f45be87405b4494161bf9ac7b25bd14de6e4d40",
            )
            .unwrap(),
            calldata: vec![
                FieldElement::from_hex_be(
                    "0x0333366346336346435623165626564653266623531386661366635396565623",
                )
                .unwrap(),
                FieldElement::from_hex_be(
                    "0x0653535393433653264616163313937353164313735393562666532383464356",
                )
                .unwrap(),
            ],
            is_query: false,
        }),
    );

    let response = deoxys
        .simulate_transactions(
            BlockId::Tag(BlockTag::Latest),
            &[max_fee_invoke_transaction],
            [SimulationFlag::SkipValidate],
        )
        .await;

    match response {
        Ok(_) => panic!("Expected a Max Fee error, but got a successful response"),
        Err(e) => {
            let error_message = format!("{:?}", e);
            assert!(
                error_message.contains("Max fee"),
                "Error do not concern Max fee"
            );
        }
    }
}

#[rstest]
#[tokio::test]
async fn fail_max_fee_too_low(deoxys: JsonRpcClient<HttpTransport>) {
    let max_fee_invoke_transaction = BroadcastedTransaction::Invoke(
        BroadcastedInvokeTransaction::V1(BroadcastedInvokeTransactionV1 {
            max_fee: FieldElement::from_hex_be("0xf").unwrap(),
            signature: vec![
                FieldElement::from_hex_be(
                    "0x5687164368262e1885f904c31bfe55362d91b9a5195d220d5d59aa3c8286349",
                )
                .expect("REASON"),
                FieldElement::from_hex_be(
                    "0x2bf8dd834492afe810152fe45083b8c768d62556f772885624ccbd52c6b80d7",
                )
                .expect("REASON"),
            ],
            nonce: FieldElement::from_hex_be("0x22").unwrap(),
            sender_address: FieldElement::from_hex_be(
                "0x019f57133d6a46990231a58a8f45be87405b4494161bf9ac7b25bd14de6e4d40",
            )
            .unwrap(),
            calldata: vec![
                FieldElement::from_hex_be(
                    "0x0333366346336346435623165626564653266623531386661366635396565623",
                )
                .unwrap(),
                FieldElement::from_hex_be(
                    "0x0653535393433653264616163313937353164313735393562666532383464356",
                )
                .unwrap(),
            ],
            is_query: false,
        }),
    );

    let response = deoxys
        .simulate_transactions(
            BlockId::Tag(BlockTag::Latest),
            &[max_fee_invoke_transaction],
            [SimulationFlag::SkipValidate],
        )
        .await;

    match response {
        Ok(_) => panic!("Expected a Max fee too low error, but got a successful response"),
        Err(e) => {
            let error_message = format!("{:?}", e);
            assert!(
                error_message.contains("Minimum fee"),
                "Error do not concern minimum fee"
            );
        }
    }
}

#[rstest]
#[tokio::test]
async fn fail_if_one_txn_cannot_be_executed(deoxys: JsonRpcClient<HttpTransport>) {
    let ok_invoke_transaction = BroadcastedTransaction::Invoke(BroadcastedInvokeTransaction::V1(
        BroadcastedInvokeTransactionV1 {
            max_fee: FieldElement::from_hex_be("0xffffffffffff").unwrap(),
            signature: vec![
                FieldElement::from_hex_be(
                    "0x5687164368262e1885f904c31bfe55362d91b9a5195d220d5d59aa3c8286349",
                )
                .expect("REASON"),
                FieldElement::from_hex_be(
                    "0x2bf8dd834492afe810152fe45083b8c768d62556f772885624ccbd52c6b80d7",
                )
                .expect("REASON"),
            ],
            nonce: FieldElement::from_hex_be("0x22").unwrap(),
            sender_address: FieldElement::from_hex_be(
                "0x019f57133d6a46990231a58a8f45be87405b4494161bf9ac7b25bd14de6e4d40",
            )
            .unwrap(),
            calldata: vec![
                FieldElement::from_hex_be(
                    "0x0333366346336346435623165626564653266623531386661366635396565623",
                )
                .unwrap(),
                FieldElement::from_hex_be(
                    "0x0653535393433653264616163313937353164313735393562666532383464356",
                )
                .unwrap(),
            ],
            is_query: false,
        },
    ));

    let bad_invoke_transaction = BroadcastedTransaction::Invoke(BroadcastedInvokeTransaction::V1(
        BroadcastedInvokeTransactionV1 {
            max_fee: FieldElement::from_hex_be("0xffffffffffff").unwrap(),
            signature: vec![
                FieldElement::from_hex_be(
                    "0x0626236383637613031643464656463376662666332333632613332313635373",
                )
                .expect("REASON"),
                FieldElement::from_hex_be(
                    "0x0386138666231633730323431643031326132323734393763346334353632343",
                )
                .expect("REASON"),
            ],
            nonce: FieldElement::from_hex_be("0x26").unwrap(),
            sender_address: FieldElement::from_hex_be(
                "0x06b0f22f1c1f96146543d4f506ce3b6f76bcf6f154ce1db4ea8e61be341f4026",
            )
            .unwrap(),
            calldata: vec![
                FieldElement::from_hex_be(
                    "0x28ad8723f66b38cab4be89d082dc21860a67e318b69e0b3adc3fc09c5bb32fa",
                )
                .unwrap(),
                FieldElement::from_hex_be(
                    "0xecfd5662af5fbcbb005e88a74bd1d7f0e5d78a4d0a278fa1744114fdd14405",
                )
                .unwrap(),
            ],
            is_query: false,
        },
    ));

    let response_deoxys = deoxys
        .simulate_transactions(
            BlockId::Tag(BlockTag::Latest),
            &[bad_invoke_transaction, ok_invoke_transaction],
            [SimulationFlag::SkipValidate],
        )
        .await;

    assert!(
        response_deoxys.is_err(),
        "Expected an error, but got a result"
    );

    let error_reason = ContractErrorData {
        revert_error: "ContractError".to_string(),
    };

    if let Err(error) = response_deoxys {
        let is_correct_error =
            checking_error_format(&error, StarknetError::ContractError(error_reason));

        assert!(
            is_correct_error,
            "Expected Contract error, but got a different error"
        );
    }
}

#[ignore = "need to submit valid fields"]
#[rstest]
#[tokio::test]
async fn works_ok_on_no_validate(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
) {
    let tx = BroadcastedTransaction::Invoke(BroadcastedInvokeTransaction::V1(
        BroadcastedInvokeTransactionV1 {
            max_fee: FieldElement::from_hex_be("0x00").unwrap(),
            signature: vec![
                FieldElement::from_hex_be(
                    "0x5687164368262e1885f904c31bfe55362d91b9a5195d220d5d59aa3c8286349",
                )
                .expect("REASON"),
                FieldElement::from_hex_be(
                    "0x2bf8dd834492afe810152fe45083b8c768d62556f772885624ccbd52c6b80d7",
                )
                .expect("REASON"),
            ],
            nonce: FieldElement::from_hex_be("0x22").unwrap(),
            sender_address: FieldElement::from_hex_be(
                "0x019f57133d6a46990231a58a8f45be87405b4494161bf9ac7b25bd14de6e4d40",
            )
            .unwrap(),
            calldata: vec![
                FieldElement::from_hex_be(
                    "0x0333366346336346435623165626564653266623531386661366635396565623",
                )
                .unwrap(),
                FieldElement::from_hex_be(
                    "0x0653535393433653264616163313937353164313735393562666532383464356",
                )
                .unwrap(),
            ],
            is_query: false,
        },
    ));

    let tx_next = tx.clone();

    let deoxys_simulations = deoxys
        .simulate_transactions(
            BlockId::Tag(BlockTag::Latest),
            &[tx],
            [SimulationFlag::SkipValidate],
        )
        .await
        .unwrap();

    let pathfinder_simulations = pathfinder
        .simulate_transactions(
            BlockId::Tag(BlockTag::Latest),
            &[tx_next],
            [SimulationFlag::SkipValidate],
        )
        .await
        .unwrap();

    // 🚨 Care : the len comparaison pass but concerning the response, there is a diff at storage entry between Juno and Pathfinder
    // Juno team is on it apparently
    assert_eq!(deoxys_simulations.len(), pathfinder_simulations.len());
    assert_eq!(deoxys_simulations, pathfinder_simulations);
}

#[ignore = "need to submit valid fields"]
#[rstest]
#[tokio::test]
async fn works_ok_on_validate_without_signature_with_skip_validate(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
) {
    let tx = BroadcastedTransaction::Invoke(BroadcastedInvokeTransaction::V1(
        BroadcastedInvokeTransactionV1 {
            max_fee: FieldElement::from_hex_be("0xffffffffffff").unwrap(),
            signature: vec![],
            nonce: FieldElement::from_hex_be("0x22").unwrap(),
            sender_address: FieldElement::from_hex_be(
                "0x019f57133d6a46990231a58a8f45be87405b4494161bf9ac7b25bd14de6e4d40",
            )
            .unwrap(),
            calldata: vec![
                FieldElement::from_hex_be(
                    "0x0333366346336346435623165626564653266623531386661366635396565623",
                )
                .unwrap(),
                FieldElement::from_hex_be(
                    "0x0653535393433653264616163313937353164313735393562666532383464356",
                )
                .unwrap(),
            ],
            is_query: false,
        },
    ));

    let tx_next = tx.clone();

    let deoxys_simulations = deoxys
        .simulate_transactions(
            BlockId::Tag(BlockTag::Latest),
            &[tx],
            [SimulationFlag::SkipValidate],
        )
        .await
        .unwrap();

    let pathfinder_simulations = pathfinder
        .simulate_transactions(
            BlockId::Tag(BlockTag::Latest),
            &[tx_next],
            [SimulationFlag::SkipValidate],
        )
        .await
        .unwrap();

    assert_eq!(deoxys_simulations.len(), pathfinder_simulations.len());
    assert_eq!(deoxys_simulations, pathfinder_simulations);
}

#[ignore = "need to submit valid fields"]
#[rstest]
#[tokio::test]
async fn works_ok_without_max_fee_with_skip_fee_charge(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
) {
    let tx = BroadcastedTransaction::Invoke(BroadcastedInvokeTransaction::V1(
        BroadcastedInvokeTransactionV1 {
            max_fee: FieldElement::from_hex_be("0x0ffffffff").unwrap(),
            signature: vec![
                FieldElement::from_hex_be(
                    "0x5687164368262e1885f904c31bfe55362d91b9a5195d220d5d59aa3c8286349",
                )
                .expect("REASON"),
                FieldElement::from_hex_be(
                    "0x2bf8dd834492afe810152fe45083b8c768d62556f772885624ccbd52c6b80d7",
                )
                .expect("REASON"),
            ],
            nonce: FieldElement::from_hex_be("0x22").unwrap(),
            sender_address: FieldElement::from_hex_be(
                "0x019f57133d6a46990231a58a8f45be87405b4494161bf9ac7b25bd14de6e4d40",
            )
            .unwrap(),
            calldata: vec![
                FieldElement::from_hex_be(
                    "0x0333366346336346435623165626564653266623531386661366635396565623",
                )
                .unwrap(),
                FieldElement::from_hex_be(
                    "0x0653535393433653264616163313937353164313735393562666532383464356",
                )
                .unwrap(),
            ],
            is_query: false,
        },
    ));

    let tx_next = tx.clone();

    let deoxys_simulations = deoxys
        .simulate_transactions(
            BlockId::Tag(BlockTag::Latest),
            &[tx],
            [SimulationFlag::SkipValidate, SimulationFlag::SkipFeeCharge],
        )
        .await
        .unwrap();

    let pathfinder_simulations = pathfinder
        .simulate_transactions(
            BlockId::Tag(BlockTag::Latest),
            &[tx_next],
            [SimulationFlag::SkipValidate, SimulationFlag::SkipFeeCharge],
        )
        .await
        .unwrap();

    assert_eq!(deoxys_simulations.len(), pathfinder_simulations.len());
    assert_eq!(deoxys_simulations, pathfinder_simulations);
}
