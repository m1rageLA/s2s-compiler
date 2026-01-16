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

    let target_type = typing::infer_expression_type(target).or_else(|| match element {
        Some(IrArrayKind::Number) => Some(IrType::Array(IrArrayKind::Number)),
        Some(IrArrayKind::Str) => Some(IrType::Array(IrArrayKind::Str)),
        Some(IrArrayKind::Bool) => Some(IrType::Array(IrArrayKind::Bool)),
        _ => None,
    });
    let index_is_number = matches!(typing::infer_expression_type(index), Some(IrType::Number));

    match target_type {
        Some(IrType::Array(IrArrayKind::Number)) if index_is_number => quote! {{
            let idx = (#index_tokens) as usize;
            #target_tokens[idx]
        }},
        Some(IrType::Array(IrArrayKind::Number)) => quote! {{
            let index_tmp = (#index_tokens).clone();
            runtime::array::index_number(&#target_tokens, index_tmp)
        }},
        Some(IrType::Array(IrArrayKind::Bool)) if index_is_number => quote! {{
            let idx = (#index_tokens) as usize;
            #target_tokens[idx]
        }},
        Some(IrType::Array(IrArrayKind::Bool)) => quote! {{
            let index_tmp = (#index_tokens).clone();
            runtime::array::index(&#target_tokens, index_tmp).to_boolean()
        }},
        Some(IrType::Array(IrArrayKind::Str)) if index_is_number => quote! {{
            let idx = (#index_tokens) as usize;
            #target_tokens[idx].clone()
        }},
        Some(IrType::Array(IrArrayKind::Str)) => quote! {{
            let index_tmp = (#index_tokens).clone();
            runtime::array::index(&#target_tokens, index_tmp).to_string()
        }},
        _ => quote! {{
            let index_tmp = (#index_tokens).clone();
            runtime::array::index(&#target_tokens, index_tmp)
        }},
    }
}
