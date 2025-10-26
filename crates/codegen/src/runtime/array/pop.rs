use ir::IrExpression;
use proc_macro2::TokenStream;
use quote::quote;
use crate::Codegen;

pub(crate) fn pop_tokens(target: &Box<IrExpression>, args: &Vec<IrExpression>) -> TokenStream {
    let target_tokens = target.codegen();
    let args_tokens: TokenStream = args
        .iter()
        .map(|arg| arg.codegen())
        .collect::<Vec<_>>()
        .into_iter()
        .collect(); // Combine into a single TokenStream
    quote! { runtime::array::pop_number(&mut #target_tokens, #args_tokens) }
}
