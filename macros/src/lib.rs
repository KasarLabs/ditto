use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn logging(_: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as ItemFn);

    input.block.stmts.insert(
        0,
        syn::parse(
            quote! {
                env_logger::builder().is_test(true).try_init().err();
            }
            .into(),
        )
        .unwrap(),
    );

    input.into_token_stream().into()
}
