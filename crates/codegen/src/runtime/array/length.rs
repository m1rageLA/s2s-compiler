use crate::Codegen;
use ir::IrExpression;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn length_tokens(target: &IrExpression) -> TokenStream {
    let target_tokens = target.codegen();
    quote! { runtime::array::length_number(&#target_tokens) }
}