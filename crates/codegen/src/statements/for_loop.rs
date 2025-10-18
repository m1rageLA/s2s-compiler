use ir::{IrExpression, IrForInit, IrStmt};
use proc_macro2::TokenStream;
use quote::quote;

use super::collect_stmt_tokens;
use crate::Codegen;

pub fn for_loop_tokens(
    init: Option<&IrForInit>,
    condition: Option<&IrExpression>,
    update: Option<&IrExpression>,
    body: &[IrStmt],
) -> TokenStream {
    let init_tokens = render_for_init(init);
    let condition_tokens = condition.map(|expr| expr.codegen());
    let update_tokens = update
        .map(|expr| {
            let expr_tokens = expr.codegen();
            quote! { #expr_tokens; }
        })
        .unwrap_or_default();
    let body_tokens = collect_stmt_tokens(body);

    if let Some(condition_tokens) = condition_tokens {
        quote! {
            {
                #init_tokens
                while #condition_tokens {
                    #(#body_tokens)*
                    #update_tokens
                }
            }
        }
    } else {
        quote! {
            {
                #init_tokens
                loop {
                    #(#body_tokens)*
                    #update_tokens
                }
            }
        }
    }
}

fn render_for_init(init: Option<&IrForInit>) -> TokenStream {
    match init {
        Some(IrForInit::VarDecl(vars)) => {
            let decls = vars.iter().map(|var| var.codegen());
            quote! { #(#decls)* }
        }
        Some(IrForInit::Expr(expr)) => {
            let expr_tokens = expr.codegen();
            quote! { #expr_tokens; }
        }
        None => TokenStream::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ir::{IrBinOp, IrExpression, IrLiteral, IrPostfixOp, IrType, IrVariable};

    #[test]
    fn for_loop_with_condition_becomes_while() {
        let init_var = IrVariable {
            name: "i".into(),
            mutable: true,
            ty: IrType::Number,
            value: Some(IrExpression::Literal(IrLiteral::Number(0.0))),
        };

        let body = vec![IrStmt::Expression(IrExpression::Identifier("i".into()))];

        let tokens = for_loop_tokens(
            Some(&IrForInit::VarDecl(vec![init_var])),
            Some(&IrExpression::Binary {
                op: IrBinOp::LessThan,
                left: Box::new(IrExpression::Identifier("i".into())),
                right: Box::new(IrExpression::Literal(IrLiteral::Number(5.0))),
            }),
            Some(&IrExpression::PostfixUnary {
                left: Box::new(IrExpression::Identifier("i".into())),
                op: IrPostfixOp::Increment,
            }),
            &body,
        );

        let expected = quote! {
        {
            let mut i : f64 = 0.0 ; while (i) < (5.0) { i ; { let ts_2_rs = i ; i += 1.0 ; ts_2_rs } ; } }
        };

        assert_eq!(tokens.to_string(), expected.to_string());
    }
}
