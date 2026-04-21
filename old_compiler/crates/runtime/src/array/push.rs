use crate::value::Value;

pub fn push(array: &mut Vec<Value>, values: Vec<Value>) -> Value {
    Value::Int(push_number(array, values) as i64)
}

pub fn push_number(array: &mut Vec<Value>, mut values: Vec<Value>) -> usize {
    array.append(&mut values);
    array.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::Value;

    #[test]
    fn test_push_adds_element_and_returns_new_length() {
        let mut array = vec![Value::Number(1.0), Value::Number(2.0)];

        let result = push_number(&mut array, vec![Value::Number(3.0)]);

        assert_eq!(result, 3);
        assert_eq!(
            array,
            vec![Value::Number(1.0), Value::Number(2.0), Value::Number(3.0)]
        );
    }

    #[test]
    fn test_push_with_different_value_types() {
        let mut array = vec![Value::String("hello".to_string()), Value::Bool(true)];

        let len = push_number(
            &mut array,
            vec![Value::Number(42.0), Value::String("rust".to_string())],
        );

        assert_eq!(len, 4);
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
        let len1 = push_number(&mut array, vec![Value::Bool(false)]);
        let len2 = push_number(&mut array, vec![Value::Bool(true), Value::Number(3.0)]);

        assert_eq!(len1, 1);
        assert_eq!(len2, 3);
        assert_eq!(
            array,
            vec![Value::Bool(false), Value::Bool(true), Value::Number(3.0)]
        );
    }
}
