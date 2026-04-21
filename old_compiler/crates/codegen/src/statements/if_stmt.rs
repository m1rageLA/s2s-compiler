use ir::{IrExpression, IrStmt};
use proc_macro2::TokenStream;
use quote::quote;

use super::collect_stmt_tokens;
use crate::Codegen;

pub fn if_tokens(
    condition: &IrExpression,
    then_branch: &[IrStmt],
    else_branch: Option<&[IrStmt]>,
) -> TokenStream {
    let condition_tokens = condition.codegen();
    let then_tokens = collect_stmt_tokens(then_branch);

    if let Some(else_branch) = else_branch {
        let else_tokens = collect_stmt_tokens(else_branch);
        quote! { if #condition_tokens { #(#then_tokens)* } else { #(#else_tokens)* } }
    } else {
        quote! { if #condition_tokens { #(#then_tokens)* } }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ir::{IrExpression, IrLiteral, IrStmt};

    #[test]
    fn emits_if_else_branches() {
        let condition = IrExpression::Literal(IrLiteral::Bool(true));
        let then_branch = vec![IrStmt::Return(None)];
        let else_branch = vec![IrStmt::Expression(IrExpression::Identifier(
            "fallback".into(),
        ))];

        let tokens = if_tokens(&condition, &then_branch, Some(&else_branch));
        let expected = quote! {
            if true {
                return ();
            } else {
                fallback;
            }
        };

        assert_eq!(tokens.to_string(), expected.to_string());
    }
}
