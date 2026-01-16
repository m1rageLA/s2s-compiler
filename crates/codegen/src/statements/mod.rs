use ir::IrStmt;
use proc_macro2::TokenStream;

use crate::Codegen;

mod block;
mod do_while;
mod control;
mod label;
mod expression;
mod for_in;
mod for_loop;
mod if_stmt;
mod labeled;
mod let_stmt;
mod return_stmt;
mod switch_stmt;
mod unsupported;
mod empty;
mod try_stmt;
mod throw_stmt;
mod var_decl;
mod while_loop;

use block::block_tokens;
use do_while::do_while_tokens;
use control::{break_tokens, continue_tokens};
use expression::expression_stmt_tokens;
use for_in::for_in_tokens;
use for_loop::for_loop_tokens;
use if_stmt::if_tokens;
use labeled::labeled_tokens;
use let_stmt::let_tokens;
use return_stmt::return_tokens;
use switch_stmt::switch_tokens;
use unsupported::unsupported_stmt;
use empty::empty_tokens;
use try_stmt::try_tokens;
use throw_stmt::throw_tokens;
use var_decl::var_decl_tokens;
use while_loop::while_tokens;

impl Codegen for IrStmt {
    type Output = TokenStream;

    fn codegen(&self) -> TokenStream {
        match self {
            IrStmt::Leteral(variable) => let_tokens(variable),
            IrStmt::Expression(expr) => expression_stmt_tokens(expr),
            IrStmt::Return(expr) => return_tokens(expr.as_ref()),
            IrStmt::Block(stmts) => block_tokens(stmts),
            IrStmt::Empty => empty_tokens(),
            IrStmt::Break(label) => break_tokens(label.as_deref()),
            IrStmt::Continue(label) => continue_tokens(label.as_deref()),
            IrStmt::Labeled { label, body } => labeled_tokens(label, body),
            IrStmt::If {
                condition,
                then_branch,
                else_branch,
            } => if_tokens(condition, then_branch, else_branch.as_deref()),
            IrStmt::While(condition, body) => while_tokens(condition, body, None),
            IrStmt::DoWhile(body, condition) => do_while_tokens(body, condition, None),
            IrStmt::For {
                init,
                condition,
                update,
                body,
            } => for_loop_tokens(init.as_ref(), condition.as_ref(), update.as_ref(), body, None),
            IrStmt::ForIn { left, right, body } => for_in_tokens(left, right, body, None),
            IrStmt::VarDecl(vars) => var_decl_tokens(vars),
            IrStmt::Switch {
                discriminant,
                cases,
            } => switch_tokens(discriminant, cases, None),
            IrStmt::Try {
                try_block,
                catch,
                finally,
            } => try_tokens(
                try_block,
                catch.as_ref(),
                finally.as_ref().map(|block| block.as_slice()),
            ),
            IrStmt::Throw(expr) => throw_tokens(expr),
            IrStmt::Unsupported(reason) => unsupported_stmt(reason),
        }
    }
}

fn collect_stmt_tokens(stmts: &[IrStmt]) -> Vec<TokenStream> {
    stmts.iter().map(|stmt| stmt.codegen()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ir::{IrExpression, IrLiteral, IrStmt, IrType, IrVariable};

    #[test]
    fn leteral_statement_delegates_to_variable_codegen() {
        let variable = IrVariable {
            name: "value".into(),
            mutable: false,
            ty: IrType::Number,
            value: Some(IrExpression::Literal(IrLiteral::Number(5.0))),
        };

        let stmt = IrStmt::Leteral(variable.clone());
        assert_eq!(stmt.codegen().to_string(), variable.codegen().to_string());
    }

    #[test]
    fn collect_stmt_tokens_preserves_statement_order() {
        let stmts = vec![
            IrStmt::Expression(IrExpression::Identifier("first".into())),
            IrStmt::Return(Some(IrExpression::Literal(IrLiteral::Bool(true)))),
        ];

        let tokens = collect_stmt_tokens(&stmts);
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].to_string(), quote::quote! { first; }.to_string());
        assert_eq!(
            tokens[1].to_string(),
            quote::quote! { return true; }.to_string()
        );
    }

    #[test]
    fn unsupported_statement_emits_panic_with_reason() {
        let stmt = IrStmt::Unsupported("not yet".into());
        assert_eq!(
            stmt.codegen().to_string(),
            quote::quote! { panic!("unsupported statement: not yet") }.to_string()
        );
    }

    #[test]
    fn var_decl_statement_batches_variable_declarations() {
        let vars = vec![
            IrVariable {
                name: "a".into(),
                mutable: false,
                ty: IrType::Number,
                value: Some(IrExpression::Literal(IrLiteral::Number(1.0))),
            },
            IrVariable {
                name: "b".into(),
                mutable: true,
                ty: IrType::Bool,
                value: None,
            },
        ];

        let stmt = IrStmt::VarDecl(vars.clone());
        let tokens = stmt.codegen();
        let expected = quote::quote! {
            let a: f64 = 1.0;
            let mut b: bool = false;
        };

        assert_eq!(tokens.to_string(), expected.to_string());
    }
}
