use ir::IrStmt;
use swc_ecma_ast::{self as ast};

use crate::expressions::expr_to_ir;

pub(crate) fn lower(do_while_stmt: &ast::DoWhileStmt) -> IrStmt {
    let body = super::stmt_block_like_to_ir(&do_while_stmt.body);
    let condition = expr_to_ir(&do_while_stmt.test);
    IrStmt::DoWhile(body, condition)
}
