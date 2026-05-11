use super::*;

pub(crate) fn array_expr_to_ir(a: &ast::ArrayLit) -> IrExpression {
    let elements = a
        .elems
        .iter()
        .filter_map(|opt| opt.as_ref())
        .map(|expr_or_spread| match expr_or_spread {
            ast::ExprOrSpread { spread: None, expr } => expr_to_ir(expr),
            ast::ExprOrSpread {
                spread: Some(_), ..
            } => IrExpression::Identifier("spread_not_supported".to_string()),
        })
        .collect();

    IrExpression::Array(elements)
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{assert_number_literal, expect_variable, lower};
    use ir::IrExpression;

    #[test]
    fn lowers_array_literals() {
        let ir_module = lower(
            r#"
            const numbers = [1, 2, 3];
        "#,
        );

        assert_eq!(ir_module.items.len(), 1);

        let numbers = expect_variable(&ir_module.items[0], "numbers");
        assert!(!numbers.mutable);
        let array = match numbers
            .value
            .as_ref()
            .expect("numbers should have initializer")
        {
            IrExpression::Array(elements) => elements,
            other => panic!("expected array literal, got {other:?}"),
        };
        assert_eq!(array.len(), 3);
        assert_number_literal(Some(&array[0]), 1.0);
        assert_number_literal(Some(&array[1]), 2.0);
        assert_number_literal(Some(&array[2]), 3.0);
    }
}
