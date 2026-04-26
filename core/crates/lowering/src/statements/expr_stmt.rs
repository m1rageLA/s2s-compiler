use ir::IrStmt;
use swc_ecma_ast::{self as ast};

use crate::expressions::expr_to_ir;

pub(crate) fn lower(expr_stmt: &ast::ExprStmt) -> IrStmt {
    let ir_expr = expr_to_ir(&expr_stmt.expr);
    IrStmt::Expression(ir_expr)
}
