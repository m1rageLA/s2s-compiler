use crate::Codegen;
use ir::{ArrayCall, IrArrayKind};
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn array_call_tokens(call: &ArrayCall) -> TokenStream {
    match call {
        ArrayCall::Push { target, args } => {
            let target_tokens = target.codegen();
            let value_tokens: Vec<TokenStream> = args.iter().map(|arg| arg.codegen()).collect();

            quote! { runtime::array::push_number(&mut #target_tokens, vec![ #( #value_tokens ),* ]) }
        }
        ArrayCall::Length { target } => {
            let target_tokens = target.codegen();
            quote! { runtime::array::length_number(&#target_tokens) }
        }
        ArrayCall::Index {
            target,
            index,
            element,
        } => {
            let target_tokens = target.codegen();
            let index_tokens = index.codegen();
            match element {
                Some(IrArrayKind::Number) => {
                    quote! { runtime::array::index_number(&#target_tokens, #index_tokens) }
                }
                _ => quote! { runtime::array::index(&#target_tokens, #index_tokens) },
            }
        }
        ArrayCall::Map { target, callback } => {
            let target_tokens = target.codegen();
            let callback_tokens = callback.codegen();
            quote! { runtime::array::map(&#target_tokens, #callback_tokens) }
        }
        ArrayCall::Filter { target, callback } => {
            let target_tokens = target.codegen();
            let callback_tokens = callback.codegen();
            quote! { runtime::array::filter(&#target_tokens, #callback_tokens) }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ir::{ArrayCall, IrExpression, IrLiteral};

    #[test]
    fn array_push_generates_mutating_runtime_call() {
        let call = ArrayCall::Push {
            target: Box::new(IrExpression::Identifier("values".into())),
            args: vec![IrExpression::RuntimeCall(ir::RuntimeNamespace::Value(
                ir::ValueCall::Coerce {
                    expr: Box::new(IrExpression::Literal(IrLiteral::Number(4.0))),
                },
            ))],
        };

        let tokens = array_call_tokens(&call);
        assert_eq!(
            tokens.to_string(),
            quote! { runtime::array::push_number(&mut values, vec![runtime::value::into_value(4.0)]) }
                .to_string()
        );
    }

    #[test]
    fn array_length_generates_runtime_call() {
        let call = ArrayCall::Length {
            target: Box::new(IrExpression::Identifier("values".into())),
        };

        let tokens = array_call_tokens(&call);
        assert_eq!(
            tokens.to_string(),
            quote! { runtime::array::length_number(&values) }.to_string()
        );
    }

    #[test]
    fn array_index_generates_runtime_call() {
        let call = ArrayCall::Index {
            target: Box::new(IrExpression::Identifier("values".into())),
            index: Box::new(IrExpression::Literal(IrLiteral::Number(1.0))),
            element: None,
        };

        let tokens = array_call_tokens(&call);
        assert_eq!(
            tokens.to_string(),
            quote! { runtime::array::index(&values, 1.0) }.to_string()
        );
    }

    #[test]
    fn array_index_for_numeric_arrays_uses_number_helper() {
        let call = ArrayCall::Index {
            target: Box::new(IrExpression::Identifier("values".into())),
            index: Box::new(IrExpression::Identifier("i".into())),
            element: Some(IrArrayKind::Number),
        };

        let tokens = array_call_tokens(&call);
        assert_eq!(
            tokens.to_string(),
            quote! { runtime::array::index_number(&values, i) }.to_string()
        );
    }

    #[test]
    fn array_map_generates_runtime_call() {
        let call = ArrayCall::Map {
            target: Box::new(IrExpression::Identifier("values".into())),
            callback: Box::new(IrExpression::Identifier("callback".into())),
        };

        let tokens = array_call_tokens(&call);
        assert_eq!(
            tokens.to_string(),
            quote! { runtime::array::map(&values, callback) }.to_string()
        );
    }

    #[test]
    fn array_filter_generates_runtime_call() {
        let call = ArrayCall::Filter {
            target: Box::new(IrExpression::Identifier("values".into())),
            callback: Box::new(IrExpression::Identifier("predicate".into())),
        };

        let tokens = array_call_tokens(&call);
        assert_eq!(
            tokens.to_string(),
            quote! { runtime::array::filter(&values, predicate) }.to_string()
        );
    }
}
