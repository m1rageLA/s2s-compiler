use ir::IrStmt;
use swc_ecma_ast as ast;

use crate::expressions::expr_to_ir;

pub(crate) fn lower(stmt: &ast::ThrowStmt) -> IrStmt {
    IrStmt::Throw(expr_to_ir(&stmt.arg))
}
