mod helpers;
use helpers::{assert_identifier, lower};
use ir::{IrExpression, IrItem, IrPostfixOp};

#[test]
fn handles_postfix_increment_and_decrement() {
    let ir_module = lower(
        r#"
        let counter = 0;
        counter++;
        counter--;
    "#,
    );

    assert_eq!(ir_module.items.len(), 3);

    let increment = match &ir_module.items[1] {
        IrItem::Expression(expr) => expr,
        other => panic!("expected expression item for increment, got {other:?}"),
    };

    match increment {
        IrExpression::PostfixUnary { left, op } => {
            assert_identifier(left.as_ref(), "counter");
            assert_eq!(*op, IrPostfixOp::Increment);
        }
        other => panic!("expected postfix increment, got {other:?}"),
    }

    let decrement = match &ir_module.items[2] {
        IrItem::Expression(expr) => expr,
        other => panic!("expected expression item for decrement, got {other:?}"),
    };

    match decrement {
        IrExpression::PostfixUnary { left, op } => {
            assert_identifier(left.as_ref(), "counter");
            assert_eq!(*op, IrPostfixOp::Decrement);
        }
        other => panic!("expected postfix decrement, got {other:?}"),
    }
}
