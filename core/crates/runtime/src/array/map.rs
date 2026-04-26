use crate::value::Value;

pub fn map<F, R>(array: &[Value], mut callback: F) -> Vec<Value>
where
    F: FnMut(Value) -> R,
    R: Into<Value>,
{
    let mut result = Vec::with_capacity(array.len());
    for value in array {
        let mapped: R = callback(Value::from(value));
        result.push(mapped.into());
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::Value;

    #[test]
    fn maps_values_using_callback() {
        let values = vec![Value::Number(1.0), Value::Number(2.0), Value::Number(3.0)];

        let mapped = map(&values, |value| match value {
            Value::Number(n) => Value::Number(n + 1.0),
            other => other,
        });

        assert_eq!(
            mapped,
            vec![Value::Number(2.0), Value::Number(3.0), Value::Number(4.0)]
        );
        // Original vector should remain untouched.
        assert_eq!(
            values,
            vec![Value::Number(1.0), Value::Number(2.0), Value::Number(3.0)]
        );
    }

    #[test]
    fn map_accepts_non_value_returns() {
        let values = vec![Value::Number(1.0), Value::Number(2.0)];

        let mapped = map(&values, |value| match value {
            Value::Number(n) => n * 3.0,
            _ => 0.0,
        });

        assert_eq!(mapped, vec![Value::Number(3.0), Value::Number(6.0)]);
    }
}
