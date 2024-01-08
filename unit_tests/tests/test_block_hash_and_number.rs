mod common;
use common::*;

use std::collections::HashMap;

use starknet_providers::{jsonrpc::{HttpTransport, JsonRpcClient}, Provider};
use starknet_core::types::{MaybePendingBlockWithTxHashes, BlockId};

#[rstest]
#[tokio::test]
async fn work_ok_at_start_and_with_new_blocks(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let alchemy = &clients[ALCHEMY];

    {
        let block_number = alchemy.block_number().await.expect("Error while getting the block number");

        let (_hash, _number) = match deoxys.get_block_with_tx_hashes(BlockId::Number(block_number)).await.unwrap() {
            MaybePendingBlockWithTxHashes::Block(b) => (b.block_hash, b.block_number),
            _ => panic!(),
        };

        let deoxys_res = deoxys.block_hash_and_number().await.expect("Deoxys : Error while getting the block hash and number");

        let alchemy_res = alchemy.block_hash_and_number().await.expect("RPC : Error while getting the block hash and number");

        assert_eq!(deoxys_res.block_hash, alchemy_res.block_hash);
        assert_eq!(deoxys_res.block_number, alchemy_res.block_number);
    }
}