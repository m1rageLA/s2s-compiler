use ir::IrStmt;
use swc_ecma_ast::{self as ast};

use crate::expressions::expr_to_ir;

pub(crate) fn lower(while_stmt: &ast::WhileStmt) -> IrStmt {
    let condition = expr_to_ir(&while_stmt.test);
    let body = super::stmt_block_like_to_ir(&while_stmt.body);
    IrStmt::While(condition, body)
}
