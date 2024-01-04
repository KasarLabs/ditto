use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use quote::quote;

#[proc_macro_derive(RpcCall)]
pub fn rpc_call_derive(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let name = input.ident;
    let expanded = quote! {
        impl RpcCall for #name {
            fn call<'b, Gclient, Gresult, Params>(
                client: &'b Gclient,
                method: &'b str,
                params: Params
            ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Gresult, jsonrpsee::core::client::Error>> + 'b>>
            where
                Gclient: jsonrpsee::core::client::ClientT + 'b,
                Gresult: RpcCall + jsonrpsee::core::DeserializeOwned + 'b,
                Params: jsonrpsee::core::traits::ToRpcParams + Send + 'b,
            {
                client.request(method, params)
            }
        }
    };

    TokenStream::from(expanded)
}