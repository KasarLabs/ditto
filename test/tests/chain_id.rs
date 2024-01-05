#[cfg(test)]
mod test_chain_id {
    use anyhow::*;
    use serde::*;
    use rpc_test_attribute::*;

    #[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
    struct BlockData(String);

    #[rpc_test(BlockData, "./unit/chainId.json")]
    fn block_data_test() {}
}
