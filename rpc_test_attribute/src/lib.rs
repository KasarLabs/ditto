use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};
use syn::parse::Parse;

struct Args {
    arg_struct: syn::Ident,
    arg_test: syn::LitStr,
}

impl Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let arg_struct = input.parse::<syn::Ident>()?;
        input.parse::<syn::Token![,]>()?;
        let arg_test = input.parse::<syn::LitStr>()?;

        Ok(Args {
            arg_struct: arg_struct,
            arg_test: arg_test,
        })
    }
}

#[proc_macro_attribute]
pub fn rpc_test(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    let args = parse_macro_input!(args as Args);

    let name = input.sig.ident;
    let arg_struct = args.arg_struct;
    let arg_test = args.arg_test;

    let expression = quote! {
        #[tokio::test]
        async fn #name() {
            // loads config and initializes RPC clients
            let config = rpc_test::test_config::TestConfig::new("./secret.json").unwrap();
            let pathfinder = jsonrpsee::http_client::HttpClientBuilder::default().build(config.pathfinder)
                .expect("Could not set up Pathfinder client");
            let deoxys = jsonrpsee::http_client::HttpClientBuilder::default().build(config.deoxys)
                .expect("Could not set up Deoxys client");

            // retrieves test parameters from json specification 
            // and generates a debug representation
            let path = #arg_test;
            let test_data = rpc_test::test_data::TestData::new(path)
                .expect(&format!("Could not retrieve test data from {}", path));
            let display_response = serde_json::to_string_pretty(&#arg_struct::default()).unwrap();
            
            for test in test_data.tests {
                // test will be run over a variety of blocks 
                // TODO: actually implement passing block_number as RPC call parameter 
                let range = match test.block_range {
                    Some(range) => range.start_inclusive..=range.stop_inclusive,
                    None => 0..=1,
                };
                let display_test = serde_json::to_string_pretty(&test).unwrap();
                let info_pathfinder = rpc_test::ClientInfo::new(
                    &pathfinder,
                    "Pathfinder",
                    &display_test,
                    &display_response,
                    &path
                );
                let info_deoxys = rpc_test::ClientInfo::new(
                    &deoxys,
                    "Deoxys",
                    &display_test,
                    &display_response,
                    &path
                );

                for _ in range {
                    // RPC calls happen *here*
                    let response_pathfinder: #arg_struct = rpc_test::client_response(&info_pathfinder, &test.cmd, &test.arg).await.unwrap();
                    let response_deoxys: #arg_struct = rpc_test::client_response(&info_deoxys, &test.cmd, &test.arg).await.unwrap();

                    assert_eq!(response_deoxys, response_pathfinder);
                }
            }
        }       
    };

    TokenStream::from(expression)
}