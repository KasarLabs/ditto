/// Block number : estimates the resources required by transactions when applied on a given state.
///
/// # Parameters
///
/// * NONE
///
/// # Expected result
///
/// * The latest block number.

#[cfg(test)]
mod block_number {
    use anyhow::*;
    use serde::*;
    use rpc_test_attribute::*;

    #[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
    struct BlockData(String);

    #[rpc_test(BlockData, "./unit/blockNumber.json")]
    fn test_block_number() {}
}
