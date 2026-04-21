use crate::{Codegen, typing};
use ir::{IrArrayKind, IrExpression, IrType, IrTypeAliasDef};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub(crate) fn index_tokens(
    target: &IrExpression,
    index: &IrExpression,
    element: Option<IrArrayKind>,
) -> TokenStream {
    if let (IrExpression::Identifier(array_name), IrExpression::Identifier(index_name)) =
        (target, index)
    {
        if let Some(alias) = typing::lookup_array_index_alias(array_name, index_name) {
            let alias_ident = format_ident!("{}", alias);
            return quote! { *#alias_ident };
        }
    }
    let target_tokens = target.codegen();
    let index_tokens = index.codegen();
    let target_ref_tokens = array_target_ref_tokens(target, &target_tokens);

    let target_type = typing::infer_expression_type(target).or_else(|| match element {
        Some(IrArrayKind::Number) => Some(IrType::Array(IrArrayKind::Number)),
        Some(IrArrayKind::Str) => Some(IrType::Array(IrArrayKind::Str)),
        Some(IrArrayKind::Bool) => Some(IrType::Array(IrArrayKind::Bool)),
        Some(IrArrayKind::Object(id)) => Some(IrType::Array(IrArrayKind::Object(id))),
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
            runtime::array::index_number(#target_ref_tokens, index_tmp)
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
            runtime::array::index(#target_ref_tokens, index_tmp).to_boolean()
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
            runtime::array::index(#target_ref_tokens, index_tmp).to_string()
        }},
        Some(IrType::Array(IrArrayKind::Object(id))) => {
            let element_copy = object_alias_is_copy(id);
            if element_copy {
                quote! {{
                    let idx = (#index_tokens) as usize;
                    #target_tokens[idx]
                }}
            } else {
                quote! {{
                    let idx = (#index_tokens) as usize;
                    #target_tokens[idx].clone()
                }}
            }
        }
        _ => quote! {{
            let index_tmp = (#index_tokens).clone();
            runtime::array::index(#target_ref_tokens, index_tmp)
        }},
    }
}

fn object_alias_is_copy(id: u32) -> bool {
    typing::lookup_type_alias(id)
        .and_then(|alias| match alias.def {
            IrTypeAliasDef::Object(fields) => Some(
                fields
                    .iter()
                    .all(|field| typing::is_copy_type(&field.ty)),
            ),
            _ => None,
        })
        .unwrap_or(false)
}

fn array_target_ref_tokens(target: &IrExpression, target_tokens: &TokenStream) -> TokenStream {
    if let IrExpression::Identifier(name) = target {
        if matches!(
            typing::lookup_binding_pass(name),
            Some(typing::ParamPass::Ref | typing::ParamPass::MutRef)
        ) {
            return quote! { #target_tokens };
        }
    }
    quote! { & #target_tokens }
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
