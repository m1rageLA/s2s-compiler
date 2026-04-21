use runtime::console::{self};
use runtime::prelude::log::ConsoleArg;
use runtime::value::Value;

#[test]
fn runtime_stringify_converts_console_args_into_values() {
    let numbers = vec![1i32, 2, 3];

    let value = ConsoleArg::to_value(&numbers);
    match value {
        Value::Array(elements) => {
            assert_eq!(elements.len(), 3);
            assert!(matches!(elements[0], Value::Number(n) if (n - 1.0).abs() < f64::EPSILON));
        }
        other => panic!("expected array value, got {other:?}"),
    }

    assert_eq!(console::log::stringify(&numbers), "123");
    assert_eq!(console::log::stringify(&true), "true");
    assert_eq!(console::log::stringify(&"hello"), "hello");
}

#[test]
fn console_arg_converts_numeric_types_to_numbers() {
    let value_i32 = ConsoleArg::to_value(&-7i32);
    let value_i64 = ConsoleArg::to_value(&123_i64);
    let value_i128 = ConsoleArg::to_value(&9_223_372_036_854_775_807_i128);
    let value_f32 = ConsoleArg::to_value(&3.5_f32);
    let value_f64 = ConsoleArg::to_value(&-0.125_f64);

    assert!(matches!(value_i32, Value::Number(n) if (n + 7.0).abs() < f64::EPSILON));
    assert!(matches!(value_i64, Value::Number(n) if (n - 123.0).abs() < f64::EPSILON));
    assert!(
        matches!(value_i128, Value::Number(n) if (n - 9_223_372_036_854_775_807_f64).abs() < 1.0)
    );
    assert!(matches!(value_f32, Value::Number(n) if (n - 3.5).abs() < f64::EPSILON));
    assert!(matches!(value_f64, Value::Number(n) if (n + 0.125).abs() < f64::EPSILON));
}

#[test]
fn console_arg_converts_collections_recursively() {
    let array = ConsoleArg::to_value(&[true, false, true]);
    let nested = ConsoleArg::to_value(&vec![vec![1i32, 2, 3], vec![4, 5]]);

    if let Value::Array(elements) = array {
        assert_eq!(elements.len(), 3);
        assert!(matches!(elements[0], Value::Bool(true)));
        assert!(matches!(elements[1], Value::Bool(false)));
    } else {
        panic!("expected array value for boolean slice");
    }

    if let Value::Array(groups) = nested {
        assert_eq!(groups.len(), 2);
        assert!(matches!(groups[0], Value::Array(ref numbers) if numbers.len() == 3));
        assert!(matches!(groups[1], Value::Array(ref numbers) if numbers.len() == 2));
    } else {
        panic!("expected nested array value");
    }
}

#[test]
fn stringify_formats_special_values_consistently() {
    assert_eq!(console::log::stringify(&()), "undefined");
    assert_eq!(console::log::stringify(&Value::Null), "null");
    assert_eq!(console::log::stringify(&Value::String("".into())), "");
    assert_eq!(console::log::stringify(&Value::Array(vec![])), "");

    let nested = Value::Array(vec![
        Value::String("foo".into()),
        Value::Array(vec![Value::Number(1.0), Value::Bool(false)]),
    ]);
    assert_eq!(console::log::stringify(&nested), "foo1false");
}
