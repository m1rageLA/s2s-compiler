use super::expr_to_ir;
use super::*;

pub fn unary_expr_to_ir(u: &ast::UnaryExpr) -> IrExpression {
    let inner = expr_to_ir(&u.arg);
    match u.op {
        ast::UnaryOp::Minus => match inner {
            IrExpression::Literal(IrLiteral::Number(value)) => {
                IrExpression::Literal(IrLiteral::Number(-value))
            }
            _ => IrExpression::Binary {
                op: IrBinOp::Sub,
                left: Box::new(IrExpression::Literal(IrLiteral::Number(0.0))),
                right: Box::new(inner),
            },
        },
        ast::UnaryOp::Plus => inner,
        _ => IrExpression::Identifier("unsupported_unary".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{assert_identifier, expect_variable, lower};
    use ir::{IrBinOp, IrExpression, IrLiteral};

    fn unwrap_value(expr: &IrExpression) -> &IrExpression {
        match expr {
            IrExpression::RuntimeCall(ir::RuntimeNamespace::Value(ir::ValueCall::Coerce {
                expr,
            })) => expr.as_ref(),
            other => other,
        }
    }

    #[test]
    fn lowers_unary_minus_and_plus() {
        let ir_module = lower(
            r#"
            const literal = -1;
            const computed = -value;
            const positive = +value;
            const unsupported = typeof value;
        "#,
        );

        assert_eq!(ir_module.items.len(), 4);

        let literal = expect_variable(&ir_module.items[0], "literal");
        match literal
            .value
            .as_ref()
            .expect("literal should have initializer")
        {
            IrExpression::Literal(IrLiteral::Number(value)) => assert_eq!(*value, -1.0),
            other => panic!("expected folded numeric literal, got {other:?}"),
        }

        let computed = expect_variable(&ir_module.items[1], "computed");
        match computed
            .value
            .as_ref()
            .expect("computed should have initializer")
        {
            IrExpression::Binary { op, left, right } => {
                assert_eq!(*op, IrBinOp::Sub);
                match left.as_ref() {
                    IrExpression::Literal(IrLiteral::Number(value)) => assert_eq!(*value, 0.0),
                    other => panic!("expected zero literal on left side, got {other:?}"),
                }
                assert_identifier(right, "value");
            }
            other => panic!("expected binary subtraction expansion, got {other:?}"),
        }

        let positive = expect_variable(&ir_module.items[2], "positive");
        let positive_value = unwrap_value(
            positive
                .value
                .as_ref()
                .expect("positive should have initializer"),
        );
        match positive_value {
            IrExpression::Identifier(name) => assert_eq!(name, "value"),
            other => panic!("expected identity for unary plus, got {other:?}"),
        }

        let unsupported = expect_variable(&ir_module.items[3], "unsupported");
        let unsupported_value = unwrap_value(
            unsupported
                .value
                .as_ref()
                .expect("unsupported should have initializer"),
        );
        match unsupported_value {
            IrExpression::Identifier(name) => assert_eq!(name, "unsupported_unary"),
            other => panic!("expected unsupported sentinel, got {other:?}"),
        }
    }

    #[test]
    fn lowers_parenthesized_expressions() {
        let ir_module = lower(
            r#"
            const result = (value);
        "#,
        );

        assert_eq!(ir_module.items.len(), 1);
        let variable = expect_variable(&ir_module.items[0], "result");
        let value = unwrap_value(
            variable
                .value
                .as_ref()
                .expect("result should have initializer"),
        );
        match value {
            IrExpression::Identifier(name) => assert_eq!(name, "value"),
            other => panic!("expected identifier after removing parentheses, got {other:?}"),
        }
    }
}
