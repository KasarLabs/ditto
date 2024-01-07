use rpc_test::test_config::TestConfig;
use starknet::providers::{
    jsonrpc::{HttpTransport, JsonRpcClient},
    Provider,
};
use url::Url;

#[tokio::test]
async fn chain_id() {
    let config =
        TestConfig::new("./secret.json").expect("'./secret.json' must contain correct node urls");

    let deoxys = JsonRpcClient::new(HttpTransport::new(
        Url::parse(&config.deoxys).expect("Error parsing deoxys url"),
    ));

    let alchemy = JsonRpcClient::new(HttpTransport::new(
        Url::parse(&config.alchemy).expect("Error parsing reference node url"),
    ));

    let response_deoxys = deoxys
        .chain_id()
        .await
        .expect("Error while getting chain id from deoxys");

    let response_reference = alchemy
        .chain_id()
        .await
        .expect("Error while getting chain id from reference node");

    assert_eq!(response_deoxys, response_reference);
}
