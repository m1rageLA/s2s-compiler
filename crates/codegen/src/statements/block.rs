use ir::IrStmt;
use proc_macro2::TokenStream;
use quote::quote;

use super::collect_stmt_tokens;
use crate::typing;

pub fn block_tokens(stmts: &[IrStmt]) -> TokenStream {
    typing::push_scope();
    let stmt_tokens = collect_stmt_tokens(stmts);
    typing::pop_scope();
    quote! { { #(#stmt_tokens)* } }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ir::{IrExpression, IrLiteral, IrStmt};

    #[test]
    fn block_wraps_statements_in_braces() {
        let stmts = vec![
            IrStmt::Expression(IrExpression::Identifier("value".into())),
            IrStmt::Return(Some(IrExpression::Literal(IrLiteral::Number(1.0)))),
        ];

        let tokens = block_tokens(&stmts);
        let expected = quote! { { value; return 1; } };

        assert_eq!(tokens.to_string(), expected.to_string());
    }
}
