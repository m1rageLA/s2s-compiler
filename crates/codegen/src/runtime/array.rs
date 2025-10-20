use crate::Codegen;
use ir::ArrayCall;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn array_call_tokens(call: &ArrayCall) -> TokenStream {
    match call {
        ArrayCall::Push { target, args } => {
            let target_tokens = target.codegen();
            let value_tokens: Vec<TokenStream> = args
                .iter()
                .map(|arg| {
                    let expr = arg.codegen();
                    quote! { runtime::value::into_value(#expr) }
                })
                .collect();

            quote! { runtime::array::push(&mut #target_tokens, vec![ #( #value_tokens ),* ]) }
        }, 
        ArrayCall::Length(_) => {
            quote! { runtime::array::length() }
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
            args: vec![IrExpression::Literal(IrLiteral::Number(4.0))],
        };

        let tokens = array_call_tokens(&call);
        assert_eq!(
            tokens.to_string(),
            quote! { runtime::array::push(&mut values, vec![runtime::value::into_value(4.0)]) }
                .to_string()
        );
    }
}
