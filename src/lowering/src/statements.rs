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
        ast::Stmt::Block(block) => {
            let inner = block_to_ir(block);
            IrStmt::Block(inner)
        }
        _ => IrStmt::Unsupported("stmt".into()),
    }
}

pub(crate) fn block_to_ir(block: &ast::BlockStmt) -> Vec<IrStmt> {
    block.stmts.iter().map(stmt_to_ir).collect()
}
