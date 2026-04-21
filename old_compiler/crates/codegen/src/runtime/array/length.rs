use crate::{Codegen, typing};
use ir::{IrArrayKind, IrExpression, IrType};
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn length_tokens(target: &IrExpression) -> TokenStream {
    let target_tokens = target.codegen();
    match typing::infer_expression_type(target) {
        Some(IrType::Array(IrArrayKind::Number))
        | Some(IrType::Array(IrArrayKind::Str))
        | Some(IrType::Array(IrArrayKind::Bool))
        | Some(IrType::Array(IrArrayKind::Object(_))) => {
            quote! { #target_tokens.len() }
        }
        _ => quote! { runtime::array::length(&#target_tokens) },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::typing;
    use quote::quote;

    #[test]
    fn typed_length_uses_usize_len() {
        typing::reset();
        typing::define("values", IrType::Array(IrArrayKind::Number));

        let tokens = length_tokens(&IrExpression::Identifier("values".into()));
        assert_eq!(tokens.to_string(), quote! { values.len() }.to_string());
    }
}
