use crate::{Codegen, typing};
use ir::{IrArrayKind, IrExpression, IrType};
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn length_tokens(target: &IrExpression) -> TokenStream {
    let target_tokens = target.codegen();
    match typing::infer_expression_type(target) {
        Some(IrType::Array(IrArrayKind::Number))
        | Some(IrType::Array(IrArrayKind::Str))
        | Some(IrType::Array(IrArrayKind::Bool)) => {
            quote! { (#target_tokens.len() as f64) }
        }
        _ => quote! { runtime::array::length(&#target_tokens) },
    }
}
