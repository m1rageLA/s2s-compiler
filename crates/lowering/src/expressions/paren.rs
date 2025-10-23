use super::*;

pub fn paren_to_ir(expr: &ast::ParenExpr) -> IrExpression {
    IrExpression::Paren(Box::new(expr_to_ir(&expr.expr)))
}
