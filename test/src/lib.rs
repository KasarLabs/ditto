#[cfg(test)]
mod tests {
    use anyhow::*;
    use serde::*;
    use rpc_call::*;
    use rpc_call_derive::*;
    use rpc_test_attribute::*;

    #[derive(Deserialize, Debug, PartialEq, RpcCall)]
    struct BlockData {
        block_hash: String,
        block_number: u32,
    }

    #[rpc_test(BlockData, "./unit/test.json")]
    fn block_data_test() {}
}