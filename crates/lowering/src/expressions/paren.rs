use super::*;

pub fn paren_to_ir(expr: &ast::ParenExpr) -> IrExpression {
    expr_to_ir(&expr.expr)
}
