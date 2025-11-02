use crate::value::Value;

pub fn join(array: &[Value], separator: Option<Value>) -> Value {
    let sep = separator
        .map(|value| value.to_string())
        .unwrap_or_else(|| ",".to_string());

    if array.is_empty() {
        return Value::String(String::new());
    }

    let mut result = String::new();
    for (index, value) in array.iter().enumerate() {
        if index > 0 {
            result.push_str(&sep);
        }
        result.push_str(&value.to_string());
    }

    Value::String(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn join_uses_comma_by_default() {
        let values = vec![
            Value::String("a".into()),
            Value::String("b".into()),
            Value::String("c".into()),
        ];

        let result = join(&values, None);
        assert_eq!(result, Value::String("a,b,c".into()));
    }

    #[test]
    fn join_respects_custom_separator() {
        let values = vec![Value::Number(1.0), Value::Number(2.0)];
        let result = join(&values, Some(Value::String(" - ".into())));
        assert_eq!(result, Value::String("1 - 2".into()));
    }

    #[test]
    fn join_on_empty_array_returns_empty_string() {
        let values: Vec<Value> = vec![];
        let result = join(&values, Some(Value::String("-".into())));
        assert_eq!(result, Value::String(String::new()));
    }
}
