#![feature(assert_matches)]

mod common;
use common::*;

use starknet::signers::{LocalWallet, SigningKey};
use starknet_accounts::{Account, Call, ConnectedAccount, Execution, SingleOwnerAccount};
use starknet_core::chain_id;
use starknet_core::types::{
    BlockId, BlockTag, BroadcastedInvokeTransaction, BroadcastedTransaction, FieldElement,
    SimulationFlag, StarknetError,
};
use starknet_core::utils::get_selector_from_name;
use starknet_providers::{
    jsonrpc::HttpTransport, JsonRpcClient, MaybeUnknownErrorCode, Provider, ProviderError,
    StarknetErrorWithMessage,
};
use std::assert_matches::assert_matches;
use std::collections::HashMap;
use unit_tests::{
    BadTransactionFactory, MaxFeeTransactionFactory, OkTransactionFactory, TransactionFactory,
};

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

type RpcAccount<'a> = SingleOwnerAccount<&'a JsonRpcClient<HttpTransport>, LocalWallet>;

pub fn build_single_owner_account<'a>(
    rpc: &'a JsonRpcClient<HttpTransport>,
    private_key: &str,
    account_address: &str,
    is_legacy: bool,
) -> RpcAccount<'a> {
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        FieldElement::from_hex_be(private_key).unwrap(),
    ));
    let account_address =
        FieldElement::from_hex_be(account_address).expect("Invalid Contract Address");
    let execution_encoding = if is_legacy {
        starknet_accounts::ExecutionEncoding::Legacy
    } else {
        starknet_accounts::ExecutionEncoding::New
    };
    SingleOwnerAccount::new(
        rpc,
        signer,
        account_address,
        chain_id::TESTNET,
        execution_encoding,
    )
}
trait PrepareInvoke {
    async fn prepare_invoke(
        &self,
        calls: Vec<Call>,
        nonce: FieldElement,
        max_fee: FieldElement,
        query_only: bool,
    ) -> BroadcastedInvokeTransaction;
}

impl PrepareInvoke for SingleOwnerAccount<&JsonRpcClient<HttpTransport>, LocalWallet> {
    async fn prepare_invoke(
        &self,
        calls: Vec<Call>,
        nonce: FieldElement,
        max_fee: FieldElement,
        query_only: bool,
    ) -> BroadcastedInvokeTransaction
    where
        Self: Account + ConnectedAccount,
    {
        let prepared_execution = Execution::new(calls, self)
            .nonce(nonce)
            .max_fee(max_fee)
            .prepared()
            .unwrap();
        prepared_execution
            .get_invoke_request(query_only)
            .await
            .unwrap()
    }
}

pub fn generate_call(
    contract_address: &str,
    function_name: &str,
    calldata_values: Vec<u8>,
) -> Call {
    let to = FieldElement::from_hex_be(contract_address).unwrap();
    let selector = get_selector_from_name(function_name).unwrap();
    let calldata = calldata_values
        .into_iter()
        .map(FieldElement::from)
        .collect();

    Call {
        to,
        selector,
        calldata,
    }
}

#[rstest]
#[tokio::test]
async fn fail_non_existing_block(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];

    let ok_invoke_transaction = OkTransactionFactory::build(Some(FieldElement::ZERO));

    assert_matches!(
        deoxys.simulate_transactions(BlockId::Hash(FieldElement::ZERO),&[ok_invoke_transaction], []).await,
        Err(ProviderError::StarknetError(StarknetErrorWithMessage { code: MaybeUnknownErrorCode::Known(code), .. })) if code == StarknetError::BlockNotFound
    );
}

#[rstest]
#[tokio::test]
async fn fail_max_fee_too_big(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];

    let max_fee_transaction = MaxFeeTransactionFactory::build(Some(FieldElement::ZERO));

    assert_matches!(
        deoxys.simulate_transactions(BlockId::Tag(BlockTag::Latest), &[max_fee_transaction], []).await,
        Err(ProviderError::StarknetError(StarknetErrorWithMessage { code: MaybeUnknownErrorCode::Unknown(500), message })) if message == "Internal server error"
    );
}

#[rstest]
#[tokio::test]
async fn fail_if_one_txn_cannot_be_executed(
    clients: HashMap<String, JsonRpcClient<HttpTransport>>,
) {
    let deoxys = &clients[DEOXYS];

    let bad_invoke_transaction = BadTransactionFactory::build(None);
    let ok_invoke_transaction = OkTransactionFactory::build(Some(FieldElement::ONE));

    assert_matches!(
        deoxys.simulate_transactions(BlockId::Tag(BlockTag::Latest),&[
            bad_invoke_transaction,
            ok_invoke_transaction,
        ],[] ).await,
        Err(ProviderError::StarknetError(StarknetErrorWithMessage { code: MaybeUnknownErrorCode::Known(code), .. })) if code == StarknetError::ContractError
    );
}

#[rstest]
#[tokio::test]
async fn works_ok_on_no_validate(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let tx = BroadcastedInvokeTransaction {
        max_fee: FieldElement::from(420u16),
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
    };

    let invoke_transaction = BroadcastedTransaction::Invoke(tx.clone());
    let invoke_transaction_2 = invoke_transaction.clone();

    let invoked_transaction = BroadcastedTransaction::Invoke(BroadcastedInvokeTransaction {
        nonce: FieldElement::ONE,
        ..tx
    });

    let invoked_transaction_2 = invoked_transaction.clone();

    let deoxys_simulations = deoxys
        .simulate_transactions(
            BlockId::Tag(BlockTag::Latest),
            &[invoke_transaction, invoked_transaction],
            [],
        )
        .await
        .unwrap();

    let pathfinder_simulations = pathfinder
        .simulate_transactions(
            BlockId::Tag(BlockTag::Latest),
            &[invoke_transaction_2, invoked_transaction_2],
            [],
        )
        .await
        .unwrap();

    assert_eq!(deoxys_simulations.len(), pathfinder_simulations.len());
    assert_eq!(
        deoxys_simulations[0].fee_estimation.gas_consumed,
        pathfinder_simulations[0].fee_estimation.gas_consumed
    );
    assert_eq!(
        deoxys_simulations[0].fee_estimation.overall_fee,
        pathfinder_simulations[0].fee_estimation.overall_fee
    );
    assert_eq!(
        deoxys_simulations[0].fee_estimation.gas_price,
        pathfinder_simulations[0].fee_estimation.gas_price
    );
}

#[rstest]
#[tokio::test]
async fn works_ok_on_validate_with_signature(
    clients: HashMap<String, JsonRpcClient<HttpTransport>>,
) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let deoxys_funding_account =
        build_single_owner_account(deoxys, SIGNER_PRIVATE, ARGENT_CONTRACT_ADDRESS, true);
    let pathfinder_funding_account =
        build_single_owner_account(pathfinder, SIGNER_PRIVATE, ARGENT_CONTRACT_ADDRESS, true);
    let deoxys_nonce = deoxys_funding_account
        .get_nonce()
        .await
        .expect("Failed to get deoxys nonce");
    let pathfinder_nonce = pathfinder_funding_account
        .get_nonce()
        .await
        .expect("Failed to get pathfinder nonce");

    let max_fee = FieldElement::from(1000u16);

    let deoxys_calls = vec![generate_call(TEST_CONTRACT_ADDRESS, "sqrt", vec![81u8])];
    let pathfinder_calls = vec![generate_call(TEST_CONTRACT_ADDRESS, "sqrt", vec![81u8])];

    let tx_deoxys = deoxys_funding_account
        .prepare_invoke(deoxys_calls, deoxys_nonce, max_fee, false)
        .await;
    let tx_pathfinder = pathfinder_funding_account
        .prepare_invoke(pathfinder_calls, pathfinder_nonce, max_fee, false)
        .await;

    let invoke_transaction_deoxys = BroadcastedTransaction::Invoke(tx_deoxys);
    let invoke_transaction_pathfinder = BroadcastedTransaction::Invoke(tx_pathfinder);

    let deoxys_simulations = deoxys
        .simulate_transactions(
            BlockId::Tag(BlockTag::Latest),
            &[invoke_transaction_deoxys],
            [],
        )
        .await
        .unwrap();
    let pathfinder_simulations = pathfinder
        .simulate_transactions(
            BlockId::Tag(BlockTag::Latest),
            &[invoke_transaction_pathfinder],
            [],
        )
        .await
        .unwrap();

    assert_eq!(deoxys_simulations.len(), pathfinder_simulations.len());
    assert_eq!(
        deoxys_simulations[0].fee_estimation.gas_consumed,
        pathfinder_simulations[0].fee_estimation.gas_consumed
    );
    assert_eq!(
        deoxys_simulations[0].fee_estimation.overall_fee,
        pathfinder_simulations[0].fee_estimation.overall_fee
    );
    assert_eq!(
        deoxys_simulations[0].fee_estimation.gas_price,
        pathfinder_simulations[0].fee_estimation.gas_price
    );
}

#[rstest]
#[tokio::test]
async fn works_ok_on_validate_without_signature_with_skip_validate(
    clients: HashMap<String, JsonRpcClient<HttpTransport>>,
) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let deoxys_funding_account =
        build_single_owner_account(&deoxys, SIGNER_PRIVATE, ARGENT_CONTRACT_ADDRESS, true);
    let pathfinder_funding_account =
        build_single_owner_account(&pathfinder, SIGNER_PRIVATE, ARGENT_CONTRACT_ADDRESS, true);
    let deoxys_nonce = deoxys_funding_account
        .get_nonce()
        .await
        .expect("Failed to get deoxys nonce");
    let pathfinder_nonce = pathfinder_funding_account
        .get_nonce()
        .await
        .expect("Failed to get pathfinder nonce");

    let max_fee = FieldElement::from(1000u16);

    let deoxys_calls = vec![generate_call(TEST_CONTRACT_ADDRESS, "sqrt", vec![81u8])];
    let pathfinder_calls = vec![generate_call(TEST_CONTRACT_ADDRESS, "sqrt", vec![81u8])];

    let tx_deoxys = deoxys_funding_account
        .prepare_invoke(deoxys_calls, deoxys_nonce, max_fee, false)
        .await;
    let tx_pathfinder = pathfinder_funding_account
        .prepare_invoke(pathfinder_calls, pathfinder_nonce, max_fee, false)
        .await;

    let invoke_transaction_deoxys = BroadcastedTransaction::Invoke(tx_deoxys);
    let invoke_transaction_pathfinder = BroadcastedTransaction::Invoke(tx_pathfinder);

    let deoxys_simulations = deoxys
        .simulate_transactions(
            BlockId::Tag(BlockTag::Latest),
            &[invoke_transaction_deoxys],
            [SimulationFlag::SkipValidate],
        )
        .await
        .unwrap();

    let pathfinder_simulations = pathfinder
        .simulate_transactions(
            BlockId::Tag(BlockTag::Latest),
            &[invoke_transaction_pathfinder],
            [SimulationFlag::SkipValidate],
        )
        .await
        .unwrap();

    assert_eq!(deoxys_simulations.len(), pathfinder_simulations.len());
    assert_eq!(
        deoxys_simulations[0].fee_estimation.gas_consumed,
        pathfinder_simulations[0].fee_estimation.gas_consumed
    );
    assert_eq!(
        deoxys_simulations[0].fee_estimation.overall_fee,
        pathfinder_simulations[0].fee_estimation.overall_fee
    );
    assert_eq!(
        deoxys_simulations[0].fee_estimation.gas_price,
        pathfinder_simulations[0].fee_estimation.gas_price
    );
}

#[rstest]
#[tokio::test]
async fn works_ok_without_max_fee_with_skip_fee_charge(
    clients: HashMap<String, JsonRpcClient<HttpTransport>>,
) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let tx = BroadcastedInvokeTransaction {
        max_fee: FieldElement::from(0u8),
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
    };

    let invoke_transaction = BroadcastedTransaction::Invoke(tx.clone());
    let invoke_transaction_2 = invoke_transaction.clone();

    let invoked_transaction_deoxys = BroadcastedTransaction::Invoke(BroadcastedInvokeTransaction {
        nonce: FieldElement::ONE,
        ..tx
    });
    let invoked_transaction_pathfinder = invoked_transaction_deoxys.clone();

    let deoxys_simulations = deoxys
        .simulate_transactions(
            BlockId::Tag(BlockTag::Latest),
            &[invoke_transaction, invoked_transaction_deoxys],
            [SimulationFlag::SkipFeeCharge],
        )
        .await
        .unwrap();

    let pathfinder_simulations = pathfinder
        .simulate_transactions(
            BlockId::Tag(BlockTag::Latest),
            &[invoke_transaction_2, invoked_transaction_pathfinder],
            [SimulationFlag::SkipFeeCharge],
        )
        .await
        .unwrap();

    assert_eq!(deoxys_simulations.len(), pathfinder_simulations.len());
    assert_eq!(
        deoxys_simulations[0].fee_estimation.gas_consumed,
        pathfinder_simulations[0].fee_estimation.gas_consumed
    );
    assert_eq!(
        deoxys_simulations[0].fee_estimation.overall_fee,
        pathfinder_simulations[0].fee_estimation.overall_fee
    );
    assert_eq!(
        deoxys_simulations[0].fee_estimation.gas_price,
        pathfinder_simulations[0].fee_estimation.gas_price
    );
}
