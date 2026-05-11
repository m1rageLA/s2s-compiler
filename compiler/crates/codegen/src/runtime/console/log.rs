use crate::Codegen;
use ir::IrExpression;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn log_tokens(args: &[IrExpression]) -> TokenStream {
    let arg_tokens: Vec<TokenStream> = args
        .iter()
        .map(|expr| {
            let expr_tokens = expr.codegen();
            quote! { runtime::console::stringify_any(&(#expr_tokens)) }
        })
        .collect();

    quote! { runtime::console::log(vec![ #( #arg_tokens ),* ]) }
}
