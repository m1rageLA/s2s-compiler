use runtime::console::{self, ConsoleArg};
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

    assert_eq!(console::stringify(&numbers), "123");
    assert_eq!(console::stringify(&true), "true");
    assert_eq!(console::stringify(&"hello"), "hello");
}
