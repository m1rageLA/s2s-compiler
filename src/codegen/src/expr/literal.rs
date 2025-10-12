use ir::IrLiteral;
use proc_macro2::{Literal, TokenStream};
use quote::quote;

use crate::Codegen;

impl Codegen for IrLiteral {
    type Output = TokenStream;

    fn codegen(&self) -> TokenStream {
        match self {
            IrLiteral::Number(value) => {
                let lit = Literal::f64_unsuffixed(*value);
                quote! { #lit }
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
        }
    }
}
