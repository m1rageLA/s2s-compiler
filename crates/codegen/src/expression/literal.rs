use ir::IrLiteral;
use proc_macro2::{Literal, TokenStream};
use quote::quote;

use crate::Codegen;

impl Codegen for IrLiteral {
    type Output = TokenStream;

    fn codegen(&self) -> TokenStream {
        match self {
            IrLiteral::Number(value) => {
                let is_uint = value.is_finite()
                    && *value >= 0.0
                    && *value <= (usize::MAX as f64)
                    && value.fract() == 0.0;
                if is_uint {
                    let use_suffix = *value > (i32::MAX as f64);
                    let lit = if use_suffix {
                        Literal::usize_suffixed(*value as usize)
                    } else {
                        Literal::usize_unsuffixed(*value as usize)
                    };
                    quote! { #lit }
                } else {
                    let lit = Literal::f64_unsuffixed(*value);
                    quote! { #lit }
                }
            }
            IrLiteral::Str(value) => {
                let lit = Literal::string(value);
                quote! { #lit.to_string() }
            }
            IrLiteral::Bool(value) => {
                if *value {
                    quote! { true }
                } else {
                    quote! { false }
                }
            }
            IrLiteral::Null => {
                quote! { runtime::value::Value::Null }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_literal_codegen_emits_unsuffixed_float() {
        let tokens = IrLiteral::Number(42.5).codegen();
        assert_eq!(
            tokens.to_string(),
            quote::quote! { 42.5 }.to_string()
        );
    }

    #[test]
    fn integer_literal_codegen_emits_usize_for_uints() {
        let tokens = IrLiteral::Number(3.0).codegen();
        assert_eq!(tokens.to_string(), quote::quote! { 3 }.to_string());
    }

    #[test]
    fn large_integer_literal_codegen_emits_usize_suffix() {
        let tokens = IrLiteral::Number(4294967296.0).codegen();
        assert_eq!(
            tokens.to_string(),
            quote::quote! { 4294967296usize }.to_string()
        );
    }

    #[test]
    fn string_literal_codegen_appends_to_string() {
        let tokens = IrLiteral::Str("hello".into()).codegen();
        assert_eq!(
            tokens.to_string(),
            quote::quote! { "hello".to_string() }.to_string()
        );
    }

    #[test]
    fn boolean_literal_codegen_emits_keyword() {
        let tokens_true = IrLiteral::Bool(true).codegen();
        let tokens_false = IrLiteral::Bool(false).codegen();

        assert_eq!(tokens_true.to_string(), quote::quote! { true }.to_string());
        assert_eq!(
            tokens_false.to_string(),
            quote::quote! { false }.to_string()
        );
    }

    #[test]
    fn null_literal_codegen_emits_runtime_value() {
        let tokens = IrLiteral::Null.codegen();
        assert_eq!(
            tokens.to_string(),
            quote::quote! { runtime::value::Value::Null }.to_string()
        );
    }
}
