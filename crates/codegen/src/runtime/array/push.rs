use crate::Codegen;
use ir::IrExpression;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn push_tokens(target: &IrExpression, args: &[IrExpression]) -> TokenStream {
    let target_tokens = target.codegen();
    let value_tokens: Vec<TokenStream> = args.iter().map(|arg| arg.codegen()).collect();
    quote! { runtime::array::push(&mut #target_tokens, vec![ #( #value_tokens ),* ]) }
}
