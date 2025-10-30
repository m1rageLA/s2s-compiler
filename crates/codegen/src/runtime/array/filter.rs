use crate::Codegen;
use ir::IrExpression;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn filter_tokens(target: &IrExpression, callback: &IrExpression) -> TokenStream {
    let target_tokens = target.codegen();
    let callback_tokens = callback.codegen();
    quote! { runtime::array::filter(&#target_tokens, #callback_tokens) }
}
