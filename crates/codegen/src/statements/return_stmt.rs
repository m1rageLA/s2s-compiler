use ir::IrExpression;
use proc_macro2::TokenStream;
use quote::quote;

use crate::Codegen;

pub fn return_tokens(expr: Option<&IrExpression>) -> TokenStream {
    match expr {
        Some(expr) => {
            let expr_tokens = expr.codegen();
            quote! { return #expr_tokens; }
        }
        None => quote! { return; },
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
        assert_eq!(tokens.to_string(), quote! { return; }.to_string());
    }
}
