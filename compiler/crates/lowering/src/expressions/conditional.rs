use super::*;

pub fn cond_expr_to_ir(cond: &ast::CondExpr) -> IrExpression {
    IrExpression::Conditional {
        test: Box::new(expr_to_ir(&cond.test)),
        consequent: Box::new(expr_to_ir(&cond.cons)),
        alternate: Box::new(expr_to_ir(&cond.alt)),
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{assert_identifier, assert_number_literal, expect_variable, lower};
    use ir::IrExpression;

    #[test]
    fn lowers_conditional_expressions() {
        let ir_module = lower(
            r#"
            const numbers = [1, 2, 3];
            let result = flag ? numbers : [0];
        "#,
        );

        assert_eq!(ir_module.items.len(), 2);

        let result = expect_variable(&ir_module.items[1], "result");
        assert!(result.mutable);
        match result
            .value
            .as_ref()
            .expect("result should have initializer")
        {
            IrExpression::Conditional {
                test,
                consequent,
                alternate,
            } => {
                assert_identifier(test, "flag");
                assert_identifier(consequent, "numbers");
                match alternate.as_ref() {
                    IrExpression::Array(elements) => {
                        assert_eq!(elements.len(), 1);
                        assert_number_literal(Some(&elements[0]), 0.0);
                    }
                    other => panic!("expected array literal in alternate branch, got {other:?}"),
                }
            }
            other => panic!("expected conditional expression, got {other:?}"),
        }
    }
}
