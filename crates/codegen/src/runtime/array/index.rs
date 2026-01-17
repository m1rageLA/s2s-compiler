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
    let index_ty = typing::infer_expression_type(index);
    let index_is_uint = matches!(index_ty, Some(IrType::UInt));
    let index_is_numeric = matches!(index_ty, Some(IrType::Number | IrType::UInt));

    match target_type {
        Some(IrType::Array(IrArrayKind::Number)) if index_is_uint => quote! {{
            #target_tokens[#index_tokens]
        }},
        Some(IrType::Array(IrArrayKind::Number)) if index_is_numeric => quote! {{
            let idx = (#index_tokens) as usize;
            #target_tokens[idx]
        }},
        Some(IrType::Array(IrArrayKind::Number)) => quote! {{
            let index_tmp = (#index_tokens).clone();
            runtime::array::index_number(&#target_tokens, index_tmp)
        }},
        Some(IrType::Array(IrArrayKind::Bool)) if index_is_uint => quote! {{
            #target_tokens[#index_tokens]
        }},
        Some(IrType::Array(IrArrayKind::Bool)) if index_is_numeric => quote! {{
            let idx = (#index_tokens) as usize;
            #target_tokens[idx]
        }},
        Some(IrType::Array(IrArrayKind::Bool)) => quote! {{
            let index_tmp = (#index_tokens).clone();
            runtime::array::index(&#target_tokens, index_tmp).to_boolean()
        }},
        Some(IrType::Array(IrArrayKind::Str)) if index_is_uint => quote! {{
            #target_tokens[#index_tokens].clone()
        }},
        Some(IrType::Array(IrArrayKind::Str)) if index_is_numeric => quote! {{
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::typing;

    fn ident(name: &str) -> IrExpression {
        IrExpression::Identifier(name.into())
    }

    #[test]
    fn indexes_number_array_with_uint_without_cast() {
        typing::reset();
        typing::define("values", IrType::Array(IrArrayKind::Number));
        typing::define("idx", IrType::UInt);

        let tokens = index_tokens(&ident("values"), &ident("idx"), Some(IrArrayKind::Number));
        let rendered = tokens.to_string().replace(' ', "");
        assert!(rendered.contains("values[idx]"));
        assert!(!rendered.contains("asusize"));
    }
}
