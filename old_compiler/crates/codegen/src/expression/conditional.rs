use ir::IrExpression;
use proc_macro2::TokenStream;
use quote::quote;

use crate::Codegen;

pub(crate) fn conditional_tokens(
    test: &IrExpression,
    consequent: &IrExpression,
    alternate: &IrExpression,
) -> TokenStream {
    let test_tokens = test.codegen();
    let consequent_tokens = consequent.codegen();
    let alternate_tokens = alternate.codegen();
    quote! { if #test_tokens { #consequent_tokens } else { #alternate_tokens } }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ir::{IrExpression, IrLiteral};
    use syn::{Expr, ExprIf, Stmt};

    fn parse_if(tokens: TokenStream) -> ExprIf {
        match syn::parse2::<Expr>(tokens).expect("conditional should parse") {
            Expr::If(expr_if) => expr_if,
            _ => panic!("expected if expression"),
        }
    }

    #[test]
    fn conditional_tokens_emits_if_else_structure() {
        let condition = IrExpression::Literal(IrLiteral::Bool(true));
        let consequent = IrExpression::Literal(IrLiteral::Number(1.0));
        let alternate = IrExpression::Literal(IrLiteral::Number(0.0));

        let expr_if = parse_if(conditional_tokens(&condition, &consequent, &alternate));

        use quote::ToTokens;
        assert_eq!(expr_if.cond.to_token_stream().to_string(), "true");

        assert!(
            matches!(expr_if.then_branch.stmts.first(), Some(Stmt::Expr(_, _))),
            "then branch should contain expression"
        );
        assert!(expr_if.else_branch.is_some(), "else branch should exist");
    }
}
