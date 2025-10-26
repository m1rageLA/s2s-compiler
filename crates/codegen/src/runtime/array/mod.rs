mod filter;
mod index;
mod length;
mod map;
mod push;
mod pop;

use pop::pop_tokens;
use filter::filter_tokens;
use index::index_tokens;
use ir::ArrayCall;
use length::length_tokens;
use map::map_tokens;
use proc_macro2::TokenStream;
use push::push_tokens;

pub(crate) fn array_call_tokens(call: &ArrayCall) -> TokenStream {
    match call {
        ArrayCall::Push { target, args } => push_tokens(target, args),
        ArrayCall::Length { target } => length_tokens(target),
        ArrayCall::Index {
            target,
            index,
            element,
        } => index_tokens(target, index, *element),
        ArrayCall::Map { target, callback } => map_tokens(target, callback),
        ArrayCall::Filter { target, callback } => filter_tokens(target, callback),
        ArrayCall::Pop { target, args } => pop_tokens(target, args),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ir::{IrArrayKind, IrExpression, IrLiteral};

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
            quote::quote! { runtime::array::push_number(&mut values, vec![runtime::value::into_value(4.0)]) }
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
            quote::quote! { runtime::array::length_number(&values) }.to_string()
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
            quote::quote! { runtime::array::index(&values, 1.0) }.to_string()
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
            quote::quote! { runtime::array::index_number(&values, i) }.to_string()
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
            quote::quote! { runtime::array::map(&values, callback) }.to_string()
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
            quote::quote! { runtime::array::filter(&values, predicate) }.to_string()
        );
    }
}
