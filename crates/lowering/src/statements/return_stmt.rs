use ir::IrStmt;
use swc_ecma_ast::{self as ast};

use crate::expressions::expr_to_ir;

pub(crate) fn lower(ret_stmt: &ast::ReturnStmt) -> IrStmt {
    let value = ret_stmt.arg.as_ref().map(|expr| expr_to_ir(expr));
    IrStmt::Return(value)
}
