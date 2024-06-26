#![feature(assert_matches)]

use constants::mainnet;
use starknet_accounts::{Account, Call, ConnectedAccount, Execution, SingleOwnerAccount};
use starknet_core::chain_id;
use starknet_core::types::BroadcastedInvokeTransaction;
use starknet_core::{
    types::{BroadcastedInvokeTransactionV1, BroadcastedTransaction, FieldElement},
    utils::get_selector_from_name,
};
use starknet_providers::{jsonrpc::HttpTransport, JsonRpcClient};
use starknet_signers::{LocalWallet, SigningKey};

pub mod constants;
pub mod fixtures;
pub mod macros;

pub trait TransactionFactory {
    fn build(nonce: Option<FieldElement>) -> BroadcastedTransaction;
}

pub struct OkTransactionFactory;

impl TransactionFactory for OkTransactionFactory {
    fn build(nonce: Option<FieldElement>) -> BroadcastedTransaction {
        BroadcastedTransaction::Invoke(BroadcastedInvokeTransaction::V1(
            BroadcastedInvokeTransactionV1 {
                max_fee: FieldElement::ZERO,
                signature: vec![],
                nonce: nonce.unwrap_or(FieldElement::ZERO),
                sender_address: FieldElement::from_hex_be(mainnet::contract::CONTRACT_ACCOUNT)
                    .unwrap(),
                calldata: vec![
                    FieldElement::from_hex_be(mainnet::contract::CONTRACT_ERC20).unwrap(),
                    get_selector_from_name("transfer").unwrap(),
                    FieldElement::from_hex_be("1").unwrap(),
                    FieldElement::from(81u8),
                ],
                is_query: true,
            },
        ))
    }
}

pub struct BadTransactionFactory;

impl TransactionFactory for BadTransactionFactory {
    fn build(_: Option<FieldElement>) -> BroadcastedTransaction {
        let transaction_v1 = BroadcastedInvokeTransactionV1 {
            max_fee: FieldElement::default(),
            nonce: FieldElement::ZERO,
            sender_address: FieldElement::default(),
            signature: vec![],
            calldata: vec![FieldElement::from_hex_be("0x0").unwrap()],
            is_query: true,
        };
        BroadcastedTransaction::Invoke(BroadcastedInvokeTransaction::V1(transaction_v1))
    }
}

pub struct MaxFeeTransactionFactory;

impl TransactionFactory for MaxFeeTransactionFactory {
    fn build(_: Option<FieldElement>) -> BroadcastedTransaction {
        let transaction_v1 = BroadcastedInvokeTransactionV1 {
            max_fee: FieldElement::from_hex_be("0x100000000000000000000000000000000").unwrap(),
            signature: vec![],
            nonce: FieldElement::ZERO,
            sender_address: FieldElement::from_hex_be(
                "0x019f57133d6a46990231a58a8f45be87405b4494161bf9ac7b25bd14de6e4d40",
            )
            .unwrap(),
            calldata: vec![
                FieldElement::from_hex_be(mainnet::contract::CONTRACT_ERC20).unwrap(),
                get_selector_from_name("sqrt").unwrap(),
                FieldElement::from_hex_be("1").unwrap(),
                FieldElement::from(81u8),
            ],
            is_query: false,
        };
        BroadcastedTransaction::Invoke(BroadcastedInvokeTransaction::V1(transaction_v1))
    }
}

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
        chain_id::SEPOLIA,
        execution_encoding,
    )
}

#[allow(async_fn_in_trait)]
pub trait PrepareInvoke {
    async fn prepare_invoke(
        &self,
        calls: Vec<Call>,
        nonce: FieldElement,
        max_fee: FieldElement,
        query_only: bool,
    ) -> BroadcastedInvokeTransactionV1;
}

impl PrepareInvoke for SingleOwnerAccount<&JsonRpcClient<HttpTransport>, LocalWallet> {
    async fn prepare_invoke(
        &self,
        calls: Vec<Call>,
        nonce: FieldElement,
        max_fee: FieldElement,
        query_only: bool,
    ) -> BroadcastedInvokeTransactionV1
    where
        Self: Account + ConnectedAccount,
    {
        let prepared_execution = Execution::new(calls, self)
            .nonce(nonce)
            .max_fee(max_fee)
            .prepared()
            .unwrap();

        let invoke_request = prepared_execution
            .get_invoke_request(query_only)
            .await
            .unwrap();

        match invoke_request {
            BroadcastedInvokeTransaction::V1(invoke_transaction) => invoke_transaction,
            BroadcastedInvokeTransaction::V3(_) => panic!("V3 transactions are not supported yet"),
        }
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
