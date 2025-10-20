use crate::value::Value;

pub fn push(array: &mut Vec<Value>, mut values: Vec<Value>) -> Value {
    array.append(&mut values);
    Value::Number(array.len() as f64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::Value;

    #[test]
    fn test_push_adds_element_and_returns_new_length() {
        let mut array = vec![Value::Number(1.0), Value::Number(2.0)];

        let result = push(&mut array, vec![Value::Number(3.0)]);

        assert_eq!(result, Value::Number(3.0));
        assert_eq!(
            array,
            vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0)
            ]
        );
    }

    #[test]
    fn test_push_with_different_value_types() {
        let mut array = vec![
            Value::String("hello".to_string()),
            Value::Bool(true),
        ];

        let len = push(
            &mut array,
            vec![
                Value::Number(42.0),
                Value::String("rust".to_string()),
            ],
        );

        assert_eq!(len, Value::Number(4.0));
        assert_eq!(
            array,
            vec![
                Value::String("hello".to_string()),
                Value::Bool(true),
                Value::Number(42.0),
                Value::String("rust".to_string()),
            ]
        );
    }

    #[test]
    fn test_push_multiple_times() {
        let mut array = vec![];
        let len1 = push(&mut array, vec![Value::Bool(false)]);
        let len2 = push(
            &mut array,
            vec![Value::Bool(true), Value::Number(3.0)],
        );

        assert_eq!(len1, Value::Number(1.0));
        assert_eq!(len2, Value::Number(3.0));
        assert_eq!(
            array,
            vec![
                Value::Bool(false),
                Value::Bool(true),
                Value::Number(3.0)
            ]
        );
    }
}
