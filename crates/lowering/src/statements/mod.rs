use ir::IrStmt;
use swc_ecma_ast::{self as ast};

mod block;
mod block_like;
mod do_while_stmt;
mod expr_stmt;
mod for_stmt;
mod if_stmt;
mod return_stmt;
mod unsupported;
mod var_decl_stmt;
mod while_stmt;

pub(crate) use block::block_to_ir;
pub(crate) use block_like::stmt_block_like_to_ir;

pub(crate) fn stmt_to_ir(stmt: &ast::Stmt) -> IrStmt {
    match stmt {
        ast::Stmt::Expr(expr_stmt) => expr_stmt::lower(expr_stmt),
        ast::Stmt::Return(ret_stmt) => return_stmt::lower(ret_stmt),
        ast::Stmt::Decl(ast::Decl::Var(var_decl)) => var_decl_stmt::lower(var_decl),
        ast::Stmt::Block(block) => block::from_block(block),
        ast::Stmt::If(if_stmt) => if_stmt::lower(if_stmt),
        ast::Stmt::While(while_stmt) => while_stmt::lower(while_stmt),
        ast::Stmt::DoWhile(do_while_stmt) => do_while_stmt::lower(do_while_stmt),
        ast::Stmt::For(for_stmt) => for_stmt::lower(for_stmt),
        _ => unsupported::lower(stmt),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{assert_identifier, assert_number_literal};
    use ir::{IrAssignOp, IrExpression, IrForInit, IrStmt, RuntimeNamespace, ValueCall};
    use swc_ecma_ast::ModuleItem;

    fn lower_first_stmt(source: &str) -> IrStmt {
        let module = parser::ast(source);
        let stmt = match module
            .body
            .first()
            .expect("expected at least one module item")
        {
            ModuleItem::Stmt(stmt) => stmt,
            other => panic!("expected statement module item, got {other:?}"),
        };
        super::stmt_to_ir(stmt)
    }

    fn lower_stmt_inside_function(body: &str) -> IrStmt {
        let source = format!(
            r#"
            function wrapper() {{
                {body}
            }}
        "#
        );

        let module = parser::ast(&source);
        let func = match module
            .body
            .first()
            .expect("expected function declaration as first item")
        {
            ModuleItem::Stmt(ast::Stmt::Decl(ast::Decl::Fn(fn_decl))) => fn_decl,
            other => panic!("expected function declaration, got {other:?}"),
        };

        let block = func
            .function
            .body
            .as_ref()
            .expect("wrapper body should exist");
        let stmt = block
            .stmts
            .first()
            .expect("expected statement in wrapper body");
        super::stmt_to_ir(stmt)
    }

    #[test]
    fn lowers_expression_statements() {
        match lower_first_stmt("value;") {
            IrStmt::Expression(expr) => assert_identifier(&expr, "value"),
            other => panic!("expected expression statement, got {other:?}"),
        }
    }

    #[test]
    fn lowers_return_with_value() {
        match lower_stmt_inside_function("return 42;") {
            IrStmt::Return(value) => assert_number_literal(value.as_ref(), 42.0),
            other => panic!("expected return statement, got {other:?}"),
        }
    }

    #[test]
    fn lowers_return_without_value() {
        match lower_stmt_inside_function("return;") {
            IrStmt::Return(value) => assert!(value.is_none()),
            other => panic!("expected empty return statement, got {other:?}"),
        }
    }

    #[test]
    fn lowers_variable_declarations() {
        match lower_first_stmt("let counter = 0;") {
            IrStmt::VarDecl(vars) => {
                assert_eq!(vars.len(), 1);
                assert_eq!(vars[0].name, "counter");
            }
            other => panic!("expected var decl statement, got {other:?}"),
        }
    }

    #[test]
    fn lowers_block_statements() {
        match lower_first_stmt("{ let flag = true; }") {
            IrStmt::Block(stmts) => {
                assert_eq!(stmts.len(), 1);
                assert!(matches!(stmts[0], IrStmt::VarDecl(_)));
            }
            other => panic!("expected block statement, got {other:?}"),
        }
    }

    #[test]
    fn lowers_if_statement_with_else_branch() {
        match lower_stmt_inside_function(
            r#"
            if (flag) {
                return 1;
            } else {
                return 2;
            }
        "#,
        ) {
            IrStmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                assert_identifier(&condition, "flag");
                assert_eq!(then_branch.len(), 1);
                assert!(matches!(then_branch[0], IrStmt::Return(_)));
                let else_branch = else_branch.expect("else branch should be present");
                assert_eq!(else_branch.len(), 1);
                assert!(matches!(else_branch[0], IrStmt::Return(_)));
            }
            other => panic!("expected if statement, got {other:?}"),
        }
    }

    #[test]
    fn lowers_if_statement_without_else_branch() {
        match lower_stmt_inside_function(
            r#"
            if (flag) {
                value();
            }
        "#,
        ) {
            IrStmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                assert_identifier(&condition, "flag");
                assert_eq!(then_branch.len(), 1);
                assert!(matches!(then_branch[0], IrStmt::Expression(_)));
                assert!(else_branch.is_none());
            }
            other => panic!("expected if statement, got {other:?}"),
        }
    }

    #[test]
    fn lowers_while_statement() {
        match lower_first_stmt("while (flag) { value(); }") {
            IrStmt::While(condition, body) => {
                assert_identifier(&condition, "flag");
                assert_eq!(body.len(), 1);
                assert!(matches!(body[0], IrStmt::Expression(_)));
            }
            other => panic!("expected while statement, got {other:?}"),
        }
    }

    #[test]
    fn lowers_do_while_statement() {
        match lower_first_stmt("do { value(); } while (flag);") {
            IrStmt::DoWhile(body, condition) => {
                assert_eq!(body.len(), 1);
                assert!(matches!(body[0], IrStmt::Expression(_)));
                assert_identifier(&condition, "flag");
            }
            other => panic!("expected do/while statement, got {other:?}"),
        }
    }

    #[test]
    fn lowers_for_statement_with_variable_initializer() {
        match lower_first_stmt("for (let i = 0; i < 1; i = i + 1) { value(); }") {
            IrStmt::For {
                init,
                condition,
                update,
                body,
            } => {
                match init.expect("init should be present") {
                    IrForInit::VarDecl(vars) => {
                        assert_eq!(vars.len(), 1);
                        assert_eq!(vars[0].name, "i");
                    }
                    other => panic!("expected variable initializer, got {other:?}"),
                }
                match condition {
                    Some(IrExpression::Binary { .. }) => {}
                    Some(IrExpression::RuntimeCall(RuntimeNamespace::Value(
                        ValueCall::LessThan { .. } | ValueCall::LessThanOrEqual { .. },
                    ))) => {}
                    other => panic!("unexpected loop condition {other:?}"),
                }
                match update.expect("update should exist") {
                    IrExpression::Assignment { op, left, right } => {
                        assert_eq!(op, IrAssignOp::Assign);
                        assert_identifier(left.as_ref(), "i");
                        match right.as_ref() {
                            IrExpression::Binary { .. }
                            | IrExpression::RuntimeCall(RuntimeNamespace::Value(
                                ValueCall::Add { .. },
                            )) => {}
                            other => panic!("expected increment expression on rhs, got {other:?}"),
                        }
                    }
                    other => panic!("expected assignment expression, got {other:?}"),
                }
                assert_eq!(body.len(), 1);
                assert!(matches!(body[0], IrStmt::Expression(_)));
            }
            other => panic!("expected for statement, got {other:?}"),
        }
    }

    #[test]
    fn lowers_for_statement_with_expression_initializer() {
        match lower_first_stmt("for (i = 0; i < 1; i = i + 1) { value(); }") {
            IrStmt::For {
                init,
                condition,
                update,
                body,
            } => {
                match init.expect("init should be present") {
                    IrForInit::Expr(expr) => match expr {
                        IrExpression::Assignment { op, left, right } => {
                            assert_eq!(op, IrAssignOp::Assign);
                            assert_identifier(left.as_ref(), "i");
                            assert_number_literal(Some(right.as_ref()), 0.0);
                        }
                        other => panic!("expected assignment initializer, got {other:?}"),
                    },
                    IrForInit::VarDecl(_) => panic!("expected expression initializer"),
                }
                match condition {
                    Some(IrExpression::Binary { .. }) => {}
                    Some(IrExpression::RuntimeCall(RuntimeNamespace::Value(
                        ValueCall::LessThan { .. } | ValueCall::LessThanOrEqual { .. },
                    ))) => {}
                    other => panic!("unexpected loop condition {other:?}"),
                }
                match update.expect("update should exist") {
                    IrExpression::Assignment { op, left, right } => {
                        assert_eq!(op, IrAssignOp::Assign);
                        assert_identifier(left.as_ref(), "i");
                        match right.as_ref() {
                            IrExpression::Binary { .. }
                            | IrExpression::RuntimeCall(RuntimeNamespace::Value(
                                ValueCall::Add { .. },
                            )) => {}
                            other => panic!("expected increment expression on rhs, got {other:?}"),
                        }
                    }
                    other => panic!("expected assignment expression, got {other:?}"),
                }
                assert_eq!(body.len(), 1);
                assert!(matches!(body[0], IrStmt::Expression(_)));
            }
            other => panic!("expected for statement, got {other:?}"),
        }
    }

    #[test]
    fn stmt_block_like_flattens_blocks() {
        let module = parser::ast(
            r#"
            {
                value();
                value();
            }
        "#,
        );
        let stmt = match module.body.first().expect("expected statement") {
            ModuleItem::Stmt(stmt) => stmt,
            _ => unreachable!(),
        };

        let lowered = super::stmt_block_like_to_ir(stmt);
        assert_eq!(lowered.len(), 2);
        assert!(matches!(lowered[0], IrStmt::Expression(_)));
    }

    #[test]
    fn stmt_block_like_wraps_non_blocks() {
        let module = parser::ast("value;");
        let stmt = match module.body.first().expect("expected statement") {
            ModuleItem::Stmt(stmt) => stmt,
            _ => unreachable!(),
        };

        let lowered = super::stmt_block_like_to_ir(stmt);
        assert_eq!(lowered.len(), 1);
        assert!(matches!(lowered[0], IrStmt::Expression(_)));
    }

    #[test]
    fn lowers_unsupported_statements() {
        match lower_first_stmt("debugger;") {
            IrStmt::Unsupported(reason) => assert_eq!(reason, "stmt"),
            other => panic!("expected unsupported statement, got {other:?}"),
        }
    }
}
