use ir::IrExpression;
use proc_macro2::TokenStream;
use quote::quote;

use crate::{typing, Codegen};

pub fn return_tokens(expr: Option<&IrExpression>) -> TokenStream {
    let expected = typing::current_return_type();
    match expr {
        Some(expr) => {
            let expr_type = typing::infer_expression_type(expr);
            let expr_tokens = expr.codegen();
            let expr_tokens = match expected {
                Some(ir::IrType::Unit) | None => expr_tokens,
                Some(ret) => typing::coerce_to_type(expr_tokens, &ret, expr_type),
            };
            quote! { return #expr_tokens; }
        }
        // Always emit an explicit unit so the return expression has a value in
        // expression-context blocks.
        None => quote! { return (); },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_with_value_includes_expression() {
        let tokens = return_tokens(Some(&IrExpression::Identifier("value".into())));
        assert_eq!(tokens.to_string(), quote! { return value; }.to_string());
    }

    #[test]
    fn return_without_value_emits_bare_return() {
        let tokens = return_tokens(None);
        assert_eq!(tokens.to_string(), quote! { return (); }.to_string());
    }
}
