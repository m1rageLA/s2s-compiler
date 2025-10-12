use ir::{IrExpression, IrStmt};
use swc_ecma_ast::{self as ast};

use crate::declarations::var_decl_to_ir;
use crate::expressions::expr_to_ir;

pub(crate) fn stmt_to_ir(stmt: &ast::Stmt) -> IrStmt {
    match stmt {
        ast::Stmt::Expr(expr_stmt) => {
            let ir_expr = expr_to_ir(&expr_stmt.expr);
            IrStmt::Expression(ir_expr)
        }
        ast::Stmt::Return(ret_stmt) => {
            let value = ret_stmt
                .arg
                .as_ref()
                .map(|expr| expr_to_ir(expr))
                .unwrap_or_else(|| IrExpression::Identifier("undefined".to_string()));
            IrStmt::Return(Some(value))
        }
        ast::Stmt::Decl(ast::Decl::Var(var_decl)) => {
            let vars = var_decl
                .decls
                .iter()
                .filter_map(|decl| var_decl_to_ir(decl))
                .collect::<Vec<_>>();
            IrStmt::VarDecl(vars)
        }
        ast::Stmt::Block(block) => IrStmt::Block(block_to_ir(block)),
        ast::Stmt::If(if_stmt) => {
            let condition = expr_to_ir(&if_stmt.test);
            let then_branch = stmt_block_like_to_ir(&if_stmt.cons);
            let else_branch = if let Some(alt) = &if_stmt.alt {
                Some(stmt_block_like_to_ir(alt))
            } else {
                None
            };
            IrStmt::If {
                condition,
                then_branch,
                else_branch,
            }
        }
        _ => IrStmt::Unsupported("stmt".into()),
    }
}

pub(crate) fn block_to_ir(block: &ast::BlockStmt) -> Vec<IrStmt> {
    block.stmts.iter().map(stmt_to_ir).collect()
}

fn stmt_block_like_to_ir(stmt: &ast::Stmt) -> Vec<IrStmt> {
    match stmt {
        ast::Stmt::Block(block) => block_to_ir(block),
        other => vec![stmt_to_ir(other)],
    }
}
