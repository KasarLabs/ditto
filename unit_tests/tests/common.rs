use starknet_core::types::StarknetError;
use starknet_core::types::{BlockId, BlockTag};
use starknet_providers::ProviderError;
use std::env;
/* Common imports used throughout all unit tests */

#[allow(unused_imports)]
pub use r#macro::*;
#[allow(unused_imports)]
pub use rstest::*;
#[allow(unused_imports)]
pub use unit_tests::constants::*;
#[allow(unused_imports)]
pub use unit_tests::fixtures::*;

/// This function aimed to check if the error is correctly handled by checking
/// the error code/type suggested by starknet rpc specs, see : https://github.com/starkware-libs/starknet-specs/blob/eedf5f899aa51a85a841333175023aa5d615aa33/api/starknet_api_openrpc.json#L3867-L3950
/// Be aware that the error message is not deeply checked, only the error type.
/// So be sure that the same contract or transaction are submitted to the function.

pub fn checking_error_format(response: &ProviderError, expected_error: StarknetError) -> bool {
    match response {
        ProviderError::StarknetError(actual_error) => {
            if *actual_error == expected_error {
                return true;
            }

            match (actual_error, &expected_error) {
                (StarknetError::ContractError(_), StarknetError::ContractError(_)) => true,

                (StarknetError::UnexpectedError(_), StarknetError::UnexpectedError(_)) => true,
                _ => false,
            }
        }
        _ => false,
    }
}

// TODO : Maybe create a function for each executions call that retrieves
// responses from the 3 differents full nodes and compare releveant fields

// If no env variable, by default we use block 100000
pub fn get_block_setting() -> BlockId {
    let block_id = env::var("BLOCK_SETTING").unwrap_or_else(|_| "100000".into());
    match block_id.as_ref() {
        "latest" => BlockId::Tag(BlockTag::Latest),
        "pending" => BlockId::Tag(BlockTag::Pending),
        _ => match block_id.parse::<u64>() {
            Ok(num) => BlockId::Number(num),
            Err(_) => BlockId::Number(100000),
        },
    }
}
