// Constants used throughout unit tests

pub const DEOXYS: &str = "deoxys";
pub const PATHFINDER: &str = "pathfinder";
pub const STARKGATE_ETH_CONTRACT_ADDR: &str = "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7";
pub const INVALID_CONTRACT_ADDR: &str = "0x4269DEADBEEF";
pub const CONTRACT_ADDR: &str = "0x03a20d4f7b4229e7c4863dab158b4d076d7f454b893d90a62011882dc4caca2a";
pub const CONTRACT_KEY: &str = "0x00f920571b9f85bdd92a867cfdc73319d0f8836f0e69e06e4c5566b6203f75cc";

///
/// Contract address for StarkGate ETH starknet bridge.
///
/// Details concerning available methods can be found on [StarkScan](https://starkscan.co/contract/0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7#read-write-contract)
///
pub const STARKGATE_ETH_BRIDGE_ADDR: &str = "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7";

///
/// Contract address for Jediswap exchange.
///
/// Details concerning available methods can be found on [StarkScan](https://starkscan.co/contract/0x041fd22b238fa21cfcf5dd45a8548974d8263b3a531a60388411c5e230f97023#read-write-contract):
///
pub const JEDI_SWAP_ADDR: &str = "0x041fd22b238fa21cfcf5dd45a8548974d8263b3a531a60388411c5e230f97023";

///
/// Contract address for Starkgate USDC on Starknet.
///
/// Details concerning this coin can be found on [StarkScan](https://starkscan.co/token/0x053c91253bc9682c04929ca02ed00b3e423f6710d2ee7e0d5ebb06f3ecf368a8):
///
pub const STARKGATE_USDC: &str = "0x053c91253bc9682c04929ca02ed00b3e423f6710d2ee7e0d5ebb06f3ecf368a8";

///
/// Contract address fpr Starkgate Ether on Starknet.
///
/// Detail concerning this coin can be found on [StarkScan](https://starkscan.co/token/0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7):
///
pub const STARKGATE_ETHER: &str = "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7";

///
/// Random ERC721 Starknet contract address.
///
/// Details concerning this contract can be found on [StarkScan](https://starkscan.co/contract/0x07fa9a8eacb89fb6cd0c7fe557e71c42a4b181ba328a9a04958136e6469c4e00)
///
pub const CONTRACT_ERC721: &str = "0x07fa9a8eacb89fb6cd0c7fe557e71c42a4b181ba328a9a04958136e6469c4e00";

///
/// Random ERC20 Starknet contract address.
///
/// Details concerning this contract can be found on [StarkScan](https://starkscan.co/contract/0x04a5fdce70877b77f03aea8a29259176f88d5bea9d0ad8c0118f5316425e6ba0)
///
pub const CONTRACT_ERC20: &str = "0x04a5fdce70877b77f03aea8a29259176f88d5bea9d0ad8c0118f5316425e6ba0";

///
/// Random ACCOUNT Starknet contract address.
///
/// Details concerning this contract can be found on [StarkScan](https://starkscan.co/contract/0x05e1eee30e79b4f592f444132526b2e2c7f505698e888c659e5f5def5a458c1a)
///
pub const CONTRACT_ACCOUNT: &str = "0x05e1eee30e79b4f592f444132526b2e2c7f505698e888c659e5f5def5a458c1a";

///
/// Random PROXY ACCOUNT Starknet contract address.
///
/// Details concerning this contract can be found on [StarkScan](https://starkscan.co/contract/0x05d7f4d55795b56ceb4dd93febe17954c7cfd5e15d7a79cb0cec067a713ac159)
///
pub const CONTRACT_ACCOUNT_PROXY: &str = "0x05d7f4d55795b56ceb4dd93febe17954c7cfd5e15d7a79cb0cec067a713ac159";

///
/// Random legacy account using Cairo v0
/// 
/// Details concerning this contract can be found on [StarkScan](https://starkscan.co/contract/0x07076931c19d0ef52b847f9412c90a2ef999ff028f1005d2d069343061762fb7)
/// 
pub const CONTRACT_LEGACY: &str = "0x07076931c19d0ef52b847f9412c90a2ef999ff028f1005d2d069343061762fb7";
pub const BLOCK_LEGACY: u64 = 2891;

///
/// Random Starknet INVOKE transaction accepted on L1
/// 
/// Details concerning this transaction can be found on [StarkScan](https://starkscan.co/tx/0x05d087d23ffb5b63f7a19ee6dfe0227d04fbcb0d0ccbfec5ec52c482429ab3f5)
/// 
pub const TRANSACTION_INVOKE: &str = "0x05d087d23ffb5b63f7a19ee6dfe0227d04fbcb0d0ccbfec5ec52c482429ab3f5";

///
/// Random Starknet L1_HANDLER transaction accepted on L1
/// 
/// Details concerning this transaction can be found on [StarkScan](https://starkscan.co/tx/0x0618051a7342133153f23df4b6d3baa6f3a933e00956b3ee621c9af76ed2cef0)
/// 
pub const TRANSACTION_L1_HANDLER: &str = "0x0618051a7342133153f23df4b6d3baa6f3a933e00956b3ee621c9af76ed2cef0";

///
/// Random Starknet DECLARE transaction accepted on L1
/// 
/// Details concerning this transaction can be found on [StarkScan](https://starkscan.co/tx/0x056c0723ef6cde62f589bbf7c5c40897b6e3d9c13e960c5e7f28276d8e9c3229)
/// 
pub const TRANSACTION_DECLARE: &str = "0x056c0723ef6cde62f589bbf7c5c40897b6e3d9c13e960c5e7f28276d8e9c3229";

///
/// Random Starknet DEPLOY transaction accepted on L1
/// 
/// Details concerning this transaction can be found on [StarkScan](https://starkscan.co/tx/0x056617d1e694083e27ecc1fcf46eb098cdeff26f223ae17612ebc746a796d9ad)
/// 
pub const TRANSACTION_DEPLOY_ACCOUNT: &str = "0x056617d1e694083e27ecc1fcf46eb098cdeff26f223ae17612ebc746a796d9ad";

///
/// Random reverted Starknet transaction
/// 
/// Details concerning this transaction can be found on [StarkScan](https://starkscan.co/tx/0x016ed559467c50c12f225f348ca8895d54b91a499ad6f856cb6086e317c120ca)
/// 
pub const TRANSACTION_REVERTED: &str = "0x016ed559467c50c12f225f348ca8895d54b91a499ad6f856cb6086e317c120ca";

pub const ACCOUNT_CONTRACT: &str = "";
pub const TEST_CONTRACT_ADDRESS: &str = "";
pub const CAIRO_1_ACCOUNT_CONTRACT_CLASS_HASH: &str = "";
pub const TEST_CONTRACT_CLASS_HASH: &str = "";