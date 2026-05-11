use ir::MathCall;
use proc_macro2::TokenStream;
use quote::quote;

use crate::{Codegen, typing};

pub(crate) fn math_call_tokens(call: &MathCall) -> TokenStream {
    match call {
        MathCall::Random => quote! { runtime::math::random_number() },
        MathCall::Sqrt { arg } => {
            let arg_tokens = arg.codegen();
            match typing::infer_expression_type(arg) {
                Some(ir::IrType::Number) => quote! { (#arg_tokens).sqrt() },
                Some(ir::IrType::UInt) => quote! { ((#arg_tokens) as f64).sqrt() },
                _ => quote! {
                    runtime::math::sqrt_number(runtime::value::into_value(#arg_tokens).into_number())
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_random_call() {
        let tokens = math_call_tokens(&MathCall::Random);

        assert_eq!(
            tokens.to_string(),
            quote::quote! { runtime::math::random_number() }.to_string()
        );
    }
}
