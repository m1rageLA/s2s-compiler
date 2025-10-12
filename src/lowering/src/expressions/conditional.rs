use super::*;

pub fn cond_expr_to_ir(cond: &ast::CondExpr) -> IrExpression {
    IrExpression::Conditional {
        test: Box::new(expr_to_ir(&cond.test)),
        consequent: Box::new(expr_to_ir(&cond.cons)),
        alternate: Box::new(expr_to_ir(&cond.alt)),
    }
}
