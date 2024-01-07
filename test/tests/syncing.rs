use rpc_test::test_config::TestConfig;
use starknet::{providers::{
    jsonrpc::{HttpTransport, JsonRpcClient},
    Provider,
}, core::types::SyncStatusType};
use url::Url;

#[tokio::test]
async fn syncing() {
    let config =
        TestConfig::new("./secret.json").expect("'./secret.json' must contain correct node urls");

    let deoxys = JsonRpcClient::new(HttpTransport::new(
        Url::parse(&config.deoxys).expect("Error parsing deoxys url"),
    ));

    let alchemy = JsonRpcClient::new(HttpTransport::new(
        Url::parse(&config.alchemy).expect("Error parsing reference node url"),
    ));

    let response_deoxys = deoxys
        .syncing()
        .await
        .expect("Error while getting sync status from deoxys");

    let response_reference = alchemy
        .syncing()
        .await
        .expect("Error while getting sync status from reference node");

    assert!(compare_sync_status(response_deoxys, response_reference));
}

fn compare_sync_status(
	a: SyncStatusType,
	b: SyncStatusType,
) -> bool {
	match (a, b) {
		(SyncStatusType::Syncing(a), SyncStatusType::Syncing(b)) => {
			a.current_block_num == b.current_block_num
			&& a.current_block_hash == b.current_block_hash
			&& a.highest_block_num == b.highest_block_num
			&& a.highest_block_hash == b.highest_block_hash
		},
		(SyncStatusType::NotSyncing, SyncStatusType::NotSyncing) => true,
		_ => false,
	}
	
}