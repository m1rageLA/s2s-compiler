use crate::{Codegen, typing};
use ir::{IrArrayKind, IrExpression, IrType};
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn index_tokens(
    target: &IrExpression,
    index: &IrExpression,
    element: Option<IrArrayKind>,
) -> TokenStream {
    let target_tokens = target.codegen();
    let index_tokens = index.codegen();

    let inferred = typing::infer_expression_type(target).or_else(|| match element {
        Some(IrArrayKind::Number) => Some(IrType::Array(IrArrayKind::Number)),
        Some(IrArrayKind::Str) => Some(IrType::Array(IrArrayKind::Str)),
        Some(IrArrayKind::Bool) => Some(IrType::Array(IrArrayKind::Bool)),
        _ => None,
    });

    match inferred {
        Some(IrType::Array(IrArrayKind::Number))
        | Some(IrType::Array(IrArrayKind::Bool)) => {
            quote! {{
                let idx = (#index_tokens) as usize;
                #target_tokens[idx]
            }}
        }
        Some(IrType::Array(IrArrayKind::Str)) => {
            quote! {{
                let idx = (#index_tokens) as usize;
                #target_tokens[idx].clone()
            }}
        }
        _ => {
            quote! {{
                let index_tmp = (#index_tokens).clone();
                runtime::array::index(&#target_tokens, index_tmp)
            }}
        }
    }
}
