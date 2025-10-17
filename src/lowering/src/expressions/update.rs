use ir::IrExpression;

use crate::expressions::expr_to_ir;

pub fn update_expr_to_ir(u: &swc_ecma_ast::UpdateExpr) -> IrExpression {
    match u.op {
        swc_ecma_ast::UpdateOp::PlusPlus => IrExpression::PostfixUnary {
            left: Box::new(expr_to_ir(&u.arg)),
            op: ir::IrPostfixOp::Increment,
        },
        swc_ecma_ast::UpdateOp::MinusMinus => IrExpression::PostfixUnary {
            left: Box::new(expr_to_ir(&u.arg)),
            op: ir::IrPostfixOp::Decrement,
        },
    }
}
