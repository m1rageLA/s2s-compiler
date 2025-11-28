use ir::IrVariable;
use proc_macro2::TokenStream;
use quote::quote;

use crate::{Codegen, typing};

pub fn var_decl_tokens(vars: &[IrVariable]) -> TokenStream {
    for var in vars {
        typing::define(&var.name, var.ty);
    }
    let decls = vars.iter().map(|var| var.codegen());
    quote! { #(#decls)* }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ir::{IrExpression, IrLiteral, IrType, IrVariable};

    #[test]
    fn concatenates_multiple_declarations() {
        let vars = vec![
            IrVariable {
                name: "a".into(),
                mutable: false,
                ty: IrType::Number,
                value: Some(IrExpression::Literal(IrLiteral::Number(1.0))),
            },
            IrVariable {
                name: "b".into(),
                mutable: true,
                ty: IrType::Bool,
                value: Some(IrExpression::Literal(IrLiteral::Bool(false))),
            },
        ];

        let tokens = var_decl_tokens(&vars);
        let expected = quote! {
            let a: f64 = 1.0;
            let mut b: bool = false;
        };

        assert_eq!(tokens.to_string(), expected.to_string());
    }
}
