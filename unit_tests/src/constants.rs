// Constants used throughout unit tests

pub const DEOXYS: &str = "deoxys";
pub const PATHFINDER: &str = "pathfinder";
pub const STARKGATE_ETH_CONTRACT_ADDR: &str =
    "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7";
pub const INVALID_CONTRACT_ADDR: &str = "0x4269DEADBEEF";
pub const CONTRACT_ADDR: &str =
    "0x03a20d4f7b4229e7c4863dab158b4d076d7f454b893d90a62011882dc4caca2a";
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
/// INVOKE transaction accepted on L1, also transaction 1 at block 5000
/// 
/// Details concerning this transaction can be found on [StarkScan](https://starkscan.co/tx/0x0214840bad3099f95c40106097b8ef5fd16f06cb23efcdaec297398f77174597)
/// 
pub const TRANSACTION_INVOKE: &str = "0x0214840bad3099f95c40106097b8ef5fd16f06cb23efcdaec297398f77174597";
pub const TRANSACTION_INVOKE_INDEX: u64 = 1;
pub const TRANSACTION_INVOKE_BLOCK_NB: u64 = 50000;

///
/// L1_HANDLER transaction accepted on L1, also transaction 57 at block 50000
/// 
/// Details concerning this transaction can be found on [StarkScan](https://starkscan.co/tx/0x05ca6470c37e943b0ae172a11359f7a457054319e8ae8771af32a1f397c7c208)
/// 
pub const TRANSACTION_L1_HANDLER: &str = "0x05ca6470c37e943b0ae172a11359f7a457054319e8ae8771af32a1f397c7c208";
pub const TRANSACTION_L1_HANDLER_INDEX: u64 = 57;
pub const TRANSACTION_L1_HANDLER_BLOCK_NB: u64 = 50000;

///
/// DECLARE transaction accepted on L1, also transaction 44 at block 49990
/// 
/// Details concerning this transaction can be found on [StarkScan](https://starkscan.co/tx/0x056c0723ef6cde62f589bbf7c5c40897b6e3d9c13e960c5e7f28276d8e9c3229)
/// 
pub const TRANSACTION_DECLARE: &str = "0x056c0723ef6cde62f589bbf7c5c40897b6e3d9c13e960c5e7f28276d8e9c3229";
pub const TRANSACTION_DECLARE_INDEX: u64 = 44;
pub const TRANSACTION_DECLARE_BLOCK_NB: u64 = 49990;

///
/// DEPLOY_ACCOUNT transaction accepted on L1, also transaction 0 at block 5000
/// 
/// Details concerning this transaction can be found on [StarkScan](https://starkscan.co/tx/0x0604e143591a6bff980f3141abdfc87f1ef3243785d251367cdaa7da5c337ba4)
/// 
pub const TRANSACTION_DEPLOY_ACCOUNT: &str = "0x0604e143591a6bff980f3141abdfc87f1ef3243785d251367cdaa7da5c337ba4";
pub const TRANSACTION_DEPLOY_ACCOUNT_INDEX: u64 = 0;
pub const TRANSACTION_DEPLOY_ACCOUNT_BLOCK_NB: u64 = 50000;

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

///
/// Value to be used as a payload for a message in the `estimate_message_fee` test.
///
pub const ETHEREUM_ADDRESS: &str = "";
pub const INVALID_ETHEREUM_ADDRESS: &str = "";
pub const SELECTOR_NAME: &str = "";