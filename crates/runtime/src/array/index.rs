use super::ArrayLike;
use crate::value::Value;

pub fn index<T, I>(array: &T, index: I) -> Value
where
    T: ArrayLike + ?Sized,
    I: Into<Value>,
{
    let index_value: Value = index.into();

    value_to_index(index_value)
        .and_then(|idx| array.get(idx))
        .unwrap_or(Value::Undefined)
}

pub fn index_number<T, I>(array: &T, position: I) -> f64
where
    T: ArrayLike + ?Sized,
    I: Into<Value>,
{
    match index(array, position) {
        Value::Number(number) => number,
        _ => panic!("expected numeric array element"),
    }
}

fn value_to_index(value: Value) -> Option<usize> {
    match value {
        Value::Number(number) => normalize_index(number),
        Value::String(string) => string.parse::<f64>().ok().and_then(normalize_index),
        _ => None,
    }
}

fn normalize_index(number: f64) -> Option<usize> {
    if !number.is_finite() {
        return None;
    }

    if number < 0.0 {
        return None;
    }

    let truncated = number.trunc();
    if truncated > (usize::MAX as f64) {
        return None;
    }

    Some(truncated as usize)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::Any;

    #[test]
    fn returns_element_for_numeric_index() {
        let array = vec![
            Value::Number(1.0),
            Value::String("value".into()),
            Value::Bool(true),
        ];

        let result = index(&array, 1.0);

        assert_eq!(result, Value::String("value".into()));
    }

    #[test]
    fn parses_string_index() {
        let array = vec![Value::Number(10.0), Value::Number(20.0)];

        let result = index(&array, "1");

        assert_eq!(result, Value::Number(20.0));
    }

    #[test]
    fn returns_undefined_for_out_of_bounds() {
        let array = vec![Value::Number(5.0)];

        let result = index(&array, 4.0);

        assert_eq!(result, Value::Undefined);
    }

    #[test]
    fn returns_undefined_for_negative_index() {
        let array = vec![Value::Number(5.0)];

        let result = index(&array, -1.0);

        assert_eq!(result, Value::Undefined);
    }

    #[test]
    fn returns_undefined_for_non_numeric_values() {
        let array = vec![Value::Number(5.0)];

        let result = index(&array, Value::Bool(true));

        assert_eq!(result, Value::Undefined);
    }

    #[test]
    fn index_number_extracts_numeric_value() {
        let array = vec![Value::Number(5.0), Value::Number(9.0)];

        let value = index_number(&array, 1.0);

        assert!((value - 9.0).abs() < f64::EPSILON);
    }

    #[test]
    #[should_panic(expected = "expected numeric array element")]
    fn index_number_panics_on_non_numeric_values() {
        let array = vec![Value::String("value".into())];

        let _ = index_number(&array, 0.0);
    }

    #[test]
    fn index_supports_boxed_arrays() {
        let boxed: Box<dyn Any> = Box::new(vec![Value::Number(3.5)]);

        let value = index_number(&boxed, 0.0);

        assert!((value - 3.5).abs() < f64::EPSILON);
    }
}
