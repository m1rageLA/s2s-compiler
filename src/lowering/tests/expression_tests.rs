mod helpers;
use helpers::{
    assert_identifier, assert_number_literal, expect_variable, lower,
};
use ir::{IrExpression, IrItem};

#[test]
fn covers_arrays_conditionals_and_member_calls() {
    let ir_module = lower(
        r#"
        const numbers = [1, 2, 3];
        let result = flag ? numbers : [0];
        foo.bar(1);
    "#,
    );

    assert_eq!(ir_module.items.len(), 3);

    let numbers = expect_variable(&ir_module.items[0], "numbers");
    assert!(!numbers.mutable);
    let array = match numbers.value.as_ref().expect("numbers should have initializer") {
        IrExpression::Array(elements) => elements,
        other => panic!("expected array literal, got {other:?}"),
    };
    assert_eq!(array.len(), 3);
    assert_number_literal(Some(&array[0]), 1.0);
    assert_number_literal(Some(&array[1]), 2.0);
    assert_number_literal(Some(&array[2]), 3.0);

    let result = expect_variable(&ir_module.items[1], "result");
    assert!(result.mutable);
    match result.value.as_ref().expect("result should have initializer") {
        IrExpression::Conditional { test, consequent, alternate } => {
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

    match &ir_module.items[2] {
        IrItem::Expression(expr) => match expr {
            IrExpression::Call { callee, args } => {
                match callee.as_ref() {
                    IrExpression::Member { object, property } => {
                        assert_identifier(object, "foo");
                        assert_eq!(property, "bar");
                    }
                    other => panic!("expected member call target, got {other:?}"),
                }
                assert_eq!(args.len(), 1);
                assert_number_literal(Some(&args[0]), 1.0);
            }
            other => panic!("expected call expression, got {other:?}"),
        },
        other => panic!("expected expression item, got {other:?}"),
    }
}
