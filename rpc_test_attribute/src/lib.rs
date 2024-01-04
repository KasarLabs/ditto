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
            let config = rpc_test::test_config::TestConfig::new("./secret.json").unwrap();
            let alchemy = jsonrpsee::http_client::HttpClientBuilder::default().build(config.alchemy)
                .with_context(|| "Could not set up Alchemy client")
                .unwrap();
            let deoxys = jsonrpsee::http_client::HttpClientBuilder::default().build(config.deoxys)
                .with_context(|| "Could not set up Deoxys client")
                .unwrap();

            let path = #arg_test;
            let test_data = rpc_test::test_data::TestData::new(path)
                .with_context(|| format!("Could not retrieve test data from {path}"))
                .unwrap();
            let display_response = serde_json::to_string_pretty(&#arg_struct::default()).unwrap();
            
            for test in test_data.tests {
                let range = match test.block_range {
                    Some(range) => range.start_inclusive..=range.stop_inclusive,
                    None => 0..=1,
                };
                let display_test = serde_json::to_string_pretty(&test).unwrap();

                for _ in range {
                    let response_alchemy: #arg_struct = #arg_struct::call(&alchemy, &test.cmd, test.arg.clone()).await
                        .with_context(|| format!(
                        "
                            Error waiting for rpc call response from Alchemy in test {path}\n\
                            RPC call: {display_test}\n\
                            Response format: {display_response}
                        "
                        ))
                        .unwrap();

                    let response_deoxys: #arg_struct = #arg_struct::call(&deoxys, &test.cmd, test.arg.clone()).await
                        .with_context(|| format!(
                        "\
                            Error waiting for rpc call response from Deoxys in test {path}\n\
                            RPC call: {display_test}\n\
                            Response format: {display_response}
                        "
                        ))
                        .unwrap();

                    assert_eq!(response_deoxys, response_alchemy);
                }
            }
        }       
    };

    TokenStream::from(expression)
}