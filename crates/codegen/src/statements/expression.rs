use ir::IrExpression;
use proc_macro2::TokenStream;
use quote::quote;

use crate::Codegen;

pub fn expression_stmt_tokens(expr: &IrExpression) -> TokenStream {
    let expr_tokens = expr.codegen();
    quote! { #expr_tokens; }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wraps_expression_with_semicolon() {
        let expr = IrExpression::Identifier("value".into());
        let tokens = expression_stmt_tokens(&expr);
        assert_eq!(tokens.to_string(), quote! { value; }.to_string());
    }
}
