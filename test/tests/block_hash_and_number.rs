/// Block hash and number : retrieves the most recent accepted block hash and number.
///
/// # Parameters
///
/// * NONE
///
/// # Expected result
///
/// * The latest block hash and number.

#[cfg(test)]
mod block_hash_and_number {
    use anyhow::*;
    use serde::*;
    use rpc_test_attribute::*;

    #[derive(Serialize, Deserialize, Debug, Default)]
    struct BlockData {
        //block_hash: String,
        block_number: u32,
    }

    impl PartialEq for BlockData {
        fn eq(&self, other: &Self) -> bool {
                self.block_number == other.block_number //TODO: Check types
        }
    }

    #[rpc_test(BlockData, "./unit/blockHashAndNumber.json")]
    fn test_block_hash_and_number() {}
}
