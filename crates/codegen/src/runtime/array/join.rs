use crate::{Codegen, typing};
use ir::{IrArrayKind, IrExpression, IrType};
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn join_tokens(target: &IrExpression, separator: Option<&IrExpression>) -> TokenStream {
    let target_tokens = target.codegen();
    let inferred = typing::infer_expression_type(target);

    match inferred {
        Some(IrType::Array(IrArrayKind::Number))
        | Some(IrType::Array(IrArrayKind::Str))
        | Some(IrType::Array(IrArrayKind::Bool)) => {
            let separator_tokens = separator
                .map(|expr| expr.codegen())
                .unwrap_or_else(|| quote! { ",".to_string() });
            quote! {{
                let sep = #separator_tokens;
                #target_tokens
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<::std::vec::Vec<_>>()
                    .join(&sep)
            }}
        }
        _ => {
            let separator_tokens = match separator {
                Some(expr) => {
                    let tokens = expr.codegen();
                    quote! { Some(runtime::value::into_value((#tokens).clone())) }
                }
                None => quote! { None },
            };
            quote! { runtime::array::join(&#target_tokens, #separator_tokens) }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ir::IrExpression;

    #[test]
    fn join_without_separator_defaults_to_none() {
        let tokens = join_tokens(&IrExpression::Identifier("values".into()), None);
        assert_eq!(
            tokens.to_string(),
            quote::quote! { runtime::array::join(&values, None) }.to_string()
        );
    }
}
