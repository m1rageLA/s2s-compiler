use ir::{IrExpression, IrStmt};
use proc_macro2::TokenStream;
use quote::quote;

use super::collect_stmt_tokens;
use crate::Codegen;

pub fn while_tokens(condition: &IrExpression, body: &[IrStmt]) -> TokenStream {
    let condition_tokens = condition.codegen();
    let body_tokens = collect_stmt_tokens(body);

    quote! { while #condition_tokens { #(#body_tokens)* } }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ir::{IrExpression, IrLiteral, IrStmt};

    #[test]
    fn emits_while_loop_structure() {
        let condition = IrExpression::Literal(IrLiteral::Bool(true));
        let body = vec![IrStmt::Return(None)];

        let tokens = while_tokens(&condition, &body);
        let expected = quote! { while true { return; } };

        assert_eq!(tokens.to_string(), expected.to_string());
    }
}
