#![feature(assert_matches)]

use constants::*;
use starknet_core::{types::{FieldElement, BroadcastedTransaction, BroadcastedInvokeTransaction}, utils::get_selector_from_name};

pub mod fixtures;
pub mod constants;
pub mod macros;

pub trait TransactionFactory {
    fn new(nonce: Option<FieldElement>) -> BroadcastedTransaction;
}

pub struct OkTransactionFactory;

impl TransactionFactory for OkTransactionFactory {
    fn new(nonce: Option<FieldElement>) -> BroadcastedTransaction {
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
    fn new(_: Option<FieldElement>) -> BroadcastedTransaction {
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