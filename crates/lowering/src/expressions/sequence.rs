use super::*;

pub(crate) fn sequence_expr_to_ir(seq: &ast::SeqExpr) -> IrExpression {
    let exprs = seq.exprs.iter().map(|expr| expr_to_ir(expr)).collect();
    IrExpression::Sequence(exprs)
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{assert_identifier, assert_number_literal, lower};
    use ir::IrExpression;

    #[test]
    fn lowers_sequence_expression() {
        let ir_module = lower(
            r#"
            const result = (first(), (second = 2), 3);
        "#,
        );

        let value = match &ir_module.items[0] {
            ir::IrItem::Variable(var) => var.value.as_ref().expect("initializer"),
            other => panic!("expected variable item, got {other:?}"),
        };

        let seq = match value {
            IrExpression::Sequence(items) => items,
            other => panic!("expected sequence expression, got {other:?}"),
        };

        assert_eq!(seq.len(), 3);
        assert_identifier(&seq[0], "first");
        match &seq[1] {
            IrExpression::Assignment { left, .. } => assert_identifier(left, "second"),
            other => panic!("expected assignment in sequence, got {other:?}"),
        }
        assert_number_literal(Some(&seq[2]), 3.0);
    }
}
