mod helpers;
use helpers::{
    assert_bool_literal, assert_number_literal, assert_string_literal, expect_variable, lower,
};
use ir::{IrBinOp, IrExpression, IrType};

#[test]
fn handles_literal_initializers() {
    let ir_module = lower(
        r#"
        let greeting: string = "hello";
        let flag: boolean = true;
        let total = 1 + 2;
        let negative = -42;
    "#,
    );

    assert_eq!(ir_module.items.len(), 4);

    let greeting = expect_variable(&ir_module.items[0], "greeting");
    assert!(greeting.mutable);
    assert_eq!(greeting.ty, IrType::Str);
    assert_string_literal(greeting.value.as_ref(), "hello");

    let flag = expect_variable(&ir_module.items[1], "flag");
    assert!(flag.mutable);
    assert_eq!(flag.ty, IrType::Bool);
    assert_bool_literal(flag.value.as_ref(), true);

    let total = expect_variable(&ir_module.items[2], "total");
    assert!(total.mutable);
    assert_eq!(total.ty, IrType::Any);
    let expr = total.value.as_ref().expect("total should have initializer");
    match expr {
        IrExpression::Binary { op, left, right } => {
            assert_eq!(*op, IrBinOp::Add);
            assert_number_literal(Some(left), 1.0);
            assert_number_literal(Some(right), 2.0);
        }
        other => panic!("expected binary expression for total, got {other:?}"),
    }

    let negative = expect_variable(&ir_module.items[3], "negative");
    assert!(negative.mutable);
    assert_number_literal(negative.value.as_ref(), -42.0);
}
