use starknet_providers::ProviderError;
use starknet_core::types::StarknetError;
use std::assert;
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

// pub fn checking_error_format(response: &ProviderError, expected_error: StarknetError) -> bool {
//     match response {
//         ProviderError::StarknetError(actual_error) => {
//             if *actual_error == expected_error {
//                 return true;
//             }

//             if let (StarknetError::ContractError(actual_data), StarknetError::ContractError(_)) = (actual_error, &expected_error) {
//                 return actual_data.revert_error.contains("Reason");
//             }

//             if let (StarknetError::UnexpectedError(actual_message), StarknetError::UnexpectedError(_)) = (actual_error, &expected_error) {
//                 return actual_message.contains("Reason");
//             }

//             false
//         },
//         _ => false,
//     }
//}

// pub fn checking_error_format(response: &ProviderError, expected_error: StarknetError) -> bool {
//     match response {
//         // Check if the response is a StarknetError
//         ProviderError::StarknetError(actual_error) => {
//             // Perform a direct comparison between the actual error and the expected error
//             *actual_error == expected_error
//         },
//         // Return false for all other types of ProviderError
//         _ => false,
//     }
// }

pub fn checking_error_format(response: &ProviderError, expected_error: StarknetError) -> bool {
    match response {
        ProviderError::StarknetError(actual_error) => {
            // Use direct comparison for most cases
            if *actual_error == expected_error {
                return true;
            }

            // Special handling for errors with additional data like ContractError
            match (actual_error, &expected_error) {
                // For ContractError, check if the error type matches, but ignore differences in the detailed message
                (StarknetError::ContractError(_), StarknetError::ContractError(_)) => true,

                // For UnexpectedError, check if the error type matches, but ignore differences in the detailed message
                (StarknetError::UnexpectedError(_), StarknetError::UnexpectedError(_)) => true,

                // Add more special cases here if needed

                // If none of the special cases match, the errors do not match
                _ => false,
            }
        },
        // The response is not a StarknetError
        _ => false,
    }
}

