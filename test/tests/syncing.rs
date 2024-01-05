#[cfg(test)]
mod test_syncing {
    use anyhow::*;
    use serde::*;
    use rpc_test_attribute::*;

    #[derive(Deserialize, Serialize, Debug, Default)]
    struct BlockData {
        starting_block_hash: String,
        starting_block_num: u64,
        current_block_hash: String,
        current_block_num: u64,
        highest_block_hash: String,
        highest_block_num: u64,
    }

    impl PartialEq for BlockData {
        fn eq(&self, other: &Self) -> bool {
                self.current_block_hash == other.current_block_hash
                && self.current_block_num == other.current_block_num
                && self.highest_block_hash == other.highest_block_hash
                && self.highest_block_num == other.highest_block_num
        }
    }

    #[rpc_test(BlockData, "./unit/syncing.json")]
    fn block_data_test() {}
}
