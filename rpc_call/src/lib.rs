use std::future::Future;
use std::pin::Pin;

use jsonrpsee::core::client::ClientT;
use jsonrpsee::core::{client, DeserializeOwned};
use jsonrpsee::core::traits::ToRpcParams;

pub trait RpcCall {
    fn call<'b, Gclient, Gresult, Params>(
        client: &'b Gclient,
        method: &'b str,
        params: Params
    ) -> Pin<Box<dyn Future<Output = Result<Gresult, client::Error>> + 'b>>
    where
        Gclient: ClientT + 'b,
        Gresult: RpcCall + DeserializeOwned + 'b,
        Params: ToRpcParams + Send + 'b
    ;
}