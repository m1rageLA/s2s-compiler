use crate::value::Value;

pub fn map<F>(array: &[Value], mut callback: F) -> Vec<Value>
where
    F: FnMut(Value) -> Value,
{
    let mut result = Vec::with_capacity(array.len());
    for value in array {
        result.push(callback(Value::from(value)));
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
}
