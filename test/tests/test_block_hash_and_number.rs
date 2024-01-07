#![feature(assert_matches)]

use rpc_test::test_config::TestConfig;
use starknet::{providers::{
    jsonrpc::{HttpTransport, JsonRpcClient},
    Provider,
}, core::types::BlockId};
use starknet::core::types::MaybePendingBlockWithTxHashes;
use url::Url;

#[tokio::test]
async fn work_ok_at_start_and_with_new_blocks() {
    let config = TestConfig::new("./secret.json").expect("'./secret.json' must contain correct node urls");
    let deoxys = JsonRpcClient::new(HttpTransport::new(
        Url::parse(&config.deoxys).unwrap()
    ));
    let alchemy = JsonRpcClient::new(HttpTransport::new(
        Url::parse(&config.alchemy).unwrap()
    ));

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