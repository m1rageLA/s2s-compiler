use crate::value::Value;

pub fn index<T>(array: &Vec<Value>, index: T) -> Value
where
    T: Into<Value>,
{
    let index_value: Value = index.into();

    value_to_index(index_value)
        .and_then(|idx| array.get(idx).cloned())
        .unwrap_or(Value::Undefined)
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
}
