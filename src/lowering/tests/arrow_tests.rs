mod helpers;
use helpers::{assert_identifier, expect_variable, lower};
use ir::{IrArrowBody, IrBinOp, IrExpression, IrParam, IrStmt, IrTemplatePart, IrType};

#[test]
fn handles_expression_and_block_bodies() {
    let ir_module = lower(
        r#"
        const double = (value: number) => value * 2;
        const format = (value: string, ...rest: number[]) => {
            return `value:${value}`;
        };
    "#,
    );

    assert_eq!(ir_module.items.len(), 2);

    let double = expect_variable(&ir_module.items[0], "double");
    assert!(!double.mutable);
    let arrow = match double.value.as_ref().expect("double should have initializer") {
        IrExpression::Arrow { params, body } => {
            assert_eq!(params, &vec![IrParam { name: "value".into(), ty: IrType::Number }]);
            body
        }
        other => panic!("expected arrow expression for double, got {other:?}"),
    };
    match arrow {
        IrArrowBody::Expr(expr) => match expr.as_ref() {
            IrExpression::Binary { op, .. } => assert_eq!(*op, IrBinOp::Mul),
            other => panic!("expected multiplication in arrow body, got {other:?}"),
        },
        other => panic!("expected expression body, got {other:?}"),
    }

    let format = expect_variable(&ir_module.items[1], "format");
    assert!(!format.mutable);
    let arrow = match format.value.as_ref().expect("format should have initializer") {
        IrExpression::Arrow { params, body } => {
            assert_eq!(params.len(), 2);
            assert_eq!(params[0], IrParam { name: "value".into(), ty: IrType::Str });
            assert_eq!(params[1].name, "rest");
            assert_eq!(params[1].ty, IrType::Any);
            body
        }
        other => panic!("expected arrow expression for format, got {other:?}"),
    };

    let block = match arrow {
        IrArrowBody::Block(stmts) => stmts,
        other => panic!("expected block arrow body, got {other:?}"),
    };

    assert_eq!(block.len(), 1);
    match &block[0] {
        IrStmt::Return(Some(expr)) => match expr {
            IrExpression::Template(parts) => {
                assert_eq!(parts.len(), 3);
                assert!(matches!(parts[0], IrTemplatePart::String(ref s) if s == "value:"));
                match &parts[1] {
                    IrTemplatePart::Expr(inner) => assert_identifier(inner, "value"),
                    other => panic!("expected interpolation expression, got {other:?}"),
                }
                assert!(matches!(parts[2], IrTemplatePart::String(ref s) if s.is_empty()));
            }
            other => panic!("expected template literal in return, got {other:?}"),
        },
        other => panic!("expected return statement in block arrow, got {other:?}"),
    }
}
