use std::collections::HashMap;

use rpc_test::test_config::TestConfig;
use rstest::fixture;
use starknet_providers::{JsonRpcClient, jsonrpc::HttpTransport};
use url::Url;

use crate::constants::DEOXYS;

// TODO: refactor this into a separate crate
#[macro_export]
macro_rules! map {
    // Match a comma-separated list of key-value pairs.
    { $( $key:expr => $value:expr ),* $(,)? } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )*
            m
        }
    };
}

#[fixture]
pub fn clients() -> HashMap<String, JsonRpcClient<HttpTransport>> {
    let config = TestConfig::new("./secret.json").expect("'./secret.json' must contain correct node urls");
    let deoxys = JsonRpcClient::new(HttpTransport::new(
        Url::parse(&config.deoxys).expect("Error parsing Deoxys node url")
    ));
    let alchemy = JsonRpcClient::new(HttpTransport::new(
        Url::parse(&config.alchemy).expect("Error parsing Deoxys node url")
    ));

    map!{
        String::from(DEOXYS) => deoxys,
        String::from("alchemy") => alchemy,
    }
}