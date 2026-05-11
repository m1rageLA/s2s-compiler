use ir::IrItem;
use swc_ecma_ast::{self as ast};

use crate::expressions::expr_to_ir;

pub(crate) fn lower(expr_stmt: &ast::ExprStmt, items: &mut Vec<IrItem>) {
    let ir_expr = expr_to_ir(&expr_stmt.expr);
    items.push(IrItem::Expression(ir_expr));
}
