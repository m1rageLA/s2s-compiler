use crate::Codegen;
use ir::IrExpression;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn coerce_tokens(expr: &IrExpression) -> TokenStream {
    let expr_tokens = expr.codegen();
    quote! { runtime::value::into_value(#expr_tokens) }
}
