use macro_utils::{extract_expr_to_str, extract_expr_to_u64, get_rpc_data, RpcData};
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

struct MacroDataRequire {
    pub block_min: u64,
    pub block_max: u64,
    pub spec_version: Option<String>,
    pub err: Result<(), Path>,
}

impl Parse for MacroDataRequire {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let args = input.parse_terminated(MetaNameValue::parse, Token![,])?;

        let mut parsed_params = Self {
            block_min: 0,
            block_max: u64::MAX,
            spec_version: None,
            err: Ok(()),
        };

        for arg in args {
            match arg.path.get_ident() {
                Some(ident) => match ident.to_string().as_str() {
                    "block_min" => {
                        parsed_params.block_min = extract_expr_to_u64(arg.value).unwrap_or(0);
                    }
                    "block_max" => {
                        parsed_params.block_max =
                            extract_expr_to_u64(arg.value).unwrap_or(u64::MAX);
                    }
                    "spec_version" => {
                        parsed_params.spec_version = match extract_expr_to_str(arg.value) {
                            Ok(s) => Some(s),
                            Err(_) => None,
                        }
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

impl MacroDataRequire {
    fn should_ignore(self, data: RpcData) -> bool {
        let Self {
            block_min,
            block_max,
            spec_version,
            err: _,
        } = self;

        (data.block_number >= block_min)
            && (data.block_number <= block_max)
            && (data.spec_version == spec_version.unwrap_or(String::from("")))
    }
}

#[proc_macro_attribute]
pub fn require(args: TokenStream, item: TokenStream) -> TokenStream {
    let block_data = get_rpc_data();
    let macro_data = parse_macro_input!(args as MacroDataRequire);

    if macro_data.should_ignore(block_data) {
        item
    } else {
        let mut func = parse_macro_input!(item as ItemFn);
        func.attrs.push(
            parse_quote!(#[ignore = "Deoxys node does not meet required specs to run this test"]),
        );

        quote!(#func).into()
    }
}
