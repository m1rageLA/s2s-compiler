use crate::value::Value;

pub fn filter<F>(array: &[Value], mut predicate: F) -> Vec<Value>
where
    F: FnMut(Value) -> bool,
{
    let mut result = Vec::new();
    for value in array {
        let candidate = Value::from(value);
        if predicate(candidate.clone()) {
            result.push(candidate);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::Value;

    #[test]
    fn filters_values_using_predicate() {
        let values = vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0),
            Value::Number(4.0),
        ];

        let filtered = filter(&values, |value| match value {
            Value::Number(n) => n % 2.0 == 0.0,
            _ => false,
        });

        assert_eq!(filtered, vec![Value::Number(2.0), Value::Number(4.0)]);
    }
}
