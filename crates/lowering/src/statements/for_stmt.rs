use ir::{IrForInit, IrStmt};
use swc_ecma_ast::{self as ast};

use crate::declarations::var_decl_to_ir;
use crate::expressions::expr_to_ir;

pub(crate) fn lower(for_stmt: &ast::ForStmt) -> IrStmt {
    let init = match &for_stmt.init {
        Some(ast::VarDeclOrExpr::VarDecl(var_decl)) => {
            let kind = var_decl.kind;
            let vars = var_decl
                .decls
                .iter()
                .filter_map(|decl| var_decl_to_ir(decl, kind))
                .collect::<Vec<_>>();
            if vars.is_empty() {
                None
            } else {
                Some(IrForInit::VarDecl(vars))
            }
        }
        Some(ast::VarDeclOrExpr::Expr(expr)) => Some(IrForInit::Expr(expr_to_ir(expr))),
        None => None,
    };

    let condition = for_stmt.test.as_ref().map(|expr| expr_to_ir(expr));
    let update = for_stmt.update.as_ref().map(|expr| expr_to_ir(expr));
    let body = super::stmt_block_like_to_ir(&for_stmt.body);

    IrStmt::For {
        init,
        condition,
        update,
        body,
    }
}
