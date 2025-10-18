use ir::{IrExpression, IrStmt};
use proc_macro2::TokenStream;
use quote::quote;

use super::collect_stmt_tokens;
use crate::Codegen;

pub fn do_while_tokens(body: &[IrStmt], condition: &IrExpression) -> TokenStream {
    let body_tokens = collect_stmt_tokens(body);
    let condition_tokens = condition.codegen();

    quote! {
        loop {
            #(#body_tokens)*
            if !(#condition_tokens) {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ir::{IrExpression, IrLiteral, IrStmt};

    #[test]
    fn do_while_wraps_body_in_loop() {
        let body = vec![IrStmt::Return(None)];
        let condition = IrExpression::Literal(IrLiteral::Bool(true));

        let tokens = do_while_tokens(&body, &condition);
        let expected = quote! {
            loop {
                return;
                if !(true) {
                    break;
                }
            }
        };

        assert_eq!(tokens.to_string(), expected.to_string());
    }
}
