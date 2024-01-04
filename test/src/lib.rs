#[cfg(test)]
mod tests {
    use anyhow::*;
    use serde::*;
    use rpc_call::*;
    use rpc_call_derive::*;
    use rpc_test_attribute::*;

    #[derive(Deserialize, Serialize, Debug, PartialEq, RpcCall, Default)]
    struct GasPriceHolder {
        price_in_wei: String,
    }

    // TODO: currently only checks against `latest` and `pending` blocks.
    // still need to add tests for specific block hash
    #[derive(Deserialize, Serialize, Debug, PartialEq, RpcCall, Default)]
    struct BlockWithTxHashes {
        block_hash: Option<String>,
        block_number: Option<u32>,
        new_root: Option<String>,
        l1_gas_price: GasPriceHolder,
        parent_hash: String,
        sequencer_address: String,
        starknet_version: String,
        status: String,
        timestamp: u32,
        transactions: Vec<String>
    }

    #[rpc_test(BlockWithTxHashes, "./unit/test_starknet_getBlockWithTxHashes.json")]
    fn test_get_block_with_tx_hashes() {}
}
