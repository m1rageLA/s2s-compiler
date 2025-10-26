use super::*;

pub fn paren_to_ir(expr: &ast::ParenExpr) -> IrExpression {
    expr_to_ir(&expr.expr)
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{expect_variable, lower};
    use ir::{IrBinOp, IrExpression};

    #[test]
    fn lowers_parenthesized_expression() {
        let ir_module = lower("const a = (1 + 2);");
        let item = &ir_module.items[0];
        let var = expect_variable(item, "a");
        let expr = var.value.as_ref().expect("expected initializer");

        match expr {
            IrExpression::Binary { op, .. } => assert_eq!(*op, IrBinOp::Add),
            other => panic!("expected binary add inside paren, got {other:?}"),
        }
    }
}
