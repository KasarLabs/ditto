#![feature(assert_matches)]

use std::{fs::File, io::Read};

use constants::*;
use serde::Deserialize;
use starknet::signers::{LocalWallet, SigningKey};
use starknet_accounts::{Account, Call, ConnectedAccount, Execution, SingleOwnerAccount};
use starknet_core::chain_id;
use starknet_core::{
    types::{BroadcastedInvokeTransaction, BroadcastedTransaction, FieldElement},
    utils::get_selector_from_name,
};
use starknet_providers::{jsonrpc::HttpTransport, JsonRpcClient};

pub mod constants;
pub mod fixtures;
pub mod macros;

#[derive(PartialEq, Debug, Deserialize)]
pub struct TestConfig {
    pub pathfinder: String,
    pub deoxys: String,
}

impl TestConfig {
    pub fn new(path: &str) -> anyhow::Result<Self> {
        let mut file = File::open(path)?;
        let mut content = String::new();

        file.read_to_string(&mut content)?;

        let config: TestConfig = serde_json::from_str(&content)
            .expect("Could not deserialize test at {path} into Config");

        Ok(config)
    }
}
pub trait TransactionFactory {
    fn build(nonce: Option<FieldElement>) -> BroadcastedTransaction;
}

pub struct OkTransactionFactory;

impl TransactionFactory for OkTransactionFactory {
    fn build(nonce: Option<FieldElement>) -> BroadcastedTransaction {
        BroadcastedTransaction::Invoke(BroadcastedInvokeTransaction {
            max_fee: FieldElement::ZERO,
            signature: vec![],
            nonce: nonce.unwrap_or(FieldElement::ZERO),
            sender_address: FieldElement::from_hex_be(ACCOUNT_CONTRACT).unwrap(),
            calldata: vec![
                FieldElement::from_hex_be(TEST_CONTRACT_ADDRESS).unwrap(),
                get_selector_from_name("sqrt").unwrap(),
                FieldElement::from_hex_be("1").unwrap(),
                FieldElement::from(81u8),
            ],
            is_query: true,
        })
    }
}

pub struct BadTransactionFactory;

impl TransactionFactory for BadTransactionFactory {
    fn build(_: Option<FieldElement>) -> BroadcastedTransaction {
        BroadcastedTransaction::Invoke(BroadcastedInvokeTransaction {
            max_fee: FieldElement::default(),
            nonce: FieldElement::ZERO,
            sender_address: FieldElement::default(),
            signature: vec![],
            calldata: vec![FieldElement::from_hex_be("0x0").unwrap()],
            is_query: true,
        })
    }
}

pub struct MaxFeeTransactionFactory;

impl TransactionFactory for MaxFeeTransactionFactory {
    fn build(_: Option<FieldElement>) -> BroadcastedTransaction {
        BroadcastedTransaction::Invoke(BroadcastedInvokeTransaction {
            max_fee: FieldElement::from_hex_be("0x100000000000000000000000000000000").unwrap(),
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
        })
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
        chain_id::TESTNET,
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
