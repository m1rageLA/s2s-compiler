use crate::Codegen;
use ir::{IrArrayKind, IrExpression};
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn index_tokens(
    target: &IrExpression,
    index: &IrExpression,
    element: Option<IrArrayKind>,
) -> TokenStream {
    let target_tokens = target.codegen();
    let index_tokens = index.codegen();
    match element {
        Some(IrArrayKind::Number) => {
            quote! { runtime::array::index_number(&#target_tokens, #index_tokens) }
        }
        _ => quote! { runtime::array::index(&#target_tokens, #index_tokens) },
    }
}
