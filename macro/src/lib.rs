use macro_utils::{extract_u64_from_expr, get_block_number};
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote, ItemFn, MetaNameValue, Path, Token,
};

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

struct ArgsRequire {
    pub block_min: u64,
    pub block_max: u64,
    pub err: Result<(), Path>,
}

impl Parse for ArgsRequire {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let args = input.parse_terminated(MetaNameValue::parse, Token![,])?;

        let mut parsed_params = Self {
            block_min: 0,
            block_max: 0,
            err: Ok(()),
        };

        for arg in args {
            match arg.path.get_ident() {
                Some(ident) => match ident.to_string().as_str() {
                    "block_min" => {
                        parsed_params.block_min = extract_u64_from_expr(arg.value).unwrap_or(0);
                    }
                    "block_max" => {
                        parsed_params.block_max =
                            extract_u64_from_expr(arg.value).unwrap_or(u64::MAX);
                    }
                    _ => {
                        parsed_params.err = Err(arg.path);
                    }
                },
                None => todo!(),
            }
        }

        Ok(parsed_params)
    }
}

#[proc_macro_attribute]
pub fn require(args: TokenStream, item: TokenStream) -> TokenStream {
    let bn = get_block_number();
    let args_parsed = parse_macro_input!(args as ArgsRequire);

    if bn >= args_parsed.block_min && bn <= args_parsed.block_max {
        item
    } else {
        let mut func = parse_macro_input!(item as ItemFn);
        func.attrs.push(
            parse_quote!(#[ignore = "Deoxys node does not meet required specs to run this test"]),
        );

        quote!(#func).into()
    }
}
