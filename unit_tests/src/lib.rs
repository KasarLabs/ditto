#![feature(assert_matches)]

use std::{fs::File, io::Read};

use constants::*;
use serde::Deserialize;
use starknet_core::{
    types::{BroadcastedInvokeTransaction, BroadcastedTransaction, FieldElement},
    utils::get_selector_from_name,
};

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
