/// Estimate Fee : estimates the resources required by transactions when applied on a given state.
///
/// # Parameters
///
/// * `request` - Arrayr of transactions to estimate
/// * `block_id` - The hash of the requested block, or number (height) of the requested block, or a block tag
///
/// # Expected result
///
/// * A sequence of fee estimations where the i'th estimate corresponds to the i'th transaction.

#[cfg(test)]
mod estimate_fee {
    use anyhow::*;
    use serde::*;
    use rpc_test_attribute::*;

    #[derive(Serialize, Deserialize, Debug, Default)]
    struct BlockData {
        gas_consumed: String,
        gas_price: String,
        overall_fee: String,
        units_consumed: String,
    } //TODO: Check data types

    impl PartialEq for BlockData {
        fn eq(&self, other: &Self) -> bool {
                self.gas_consumed == other.gas_consumed
                && self.gas_price == other.gas_price
                && self.overall_fee == other.overall_fee
                && self.units_consumed == other.units_consumed
        }
    }
    #[rpc_test(BlockData, "./unit/estimateFee.json")]
    fn test_estimate_fee() {}
}