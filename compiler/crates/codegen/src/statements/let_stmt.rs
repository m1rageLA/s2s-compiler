use ir::IrVariable;
use proc_macro2::TokenStream;

use crate::Codegen;

pub fn let_tokens(variable: &IrVariable) -> TokenStream {
    variable.codegen()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ir::{IrExpression, IrLiteral, IrType, IrVariable};

    #[test]
    fn emits_typed_let_binding() {
        let variable = IrVariable {
            name: "value".into(),
            mutable: false,
            ty: IrType::Number,
            value: Some(IrExpression::Literal(IrLiteral::Number(1.0))),
        };

        let tokens = let_tokens(&variable);
        assert_eq!(
            tokens.to_string(),
            quote::quote! { let value: f64 = (1) as f64; }.to_string()
        );
    }
}
