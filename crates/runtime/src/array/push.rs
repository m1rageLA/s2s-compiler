use crate::value::Value;
pub fn push(array: &mut Vec<Value>, value: Value) -> Value {
    array.push(value);
    return Value::Number(array.len() as f64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::Value;

    #[test]
    fn test_push_adds_element_and_returns_new_length() {
        // Исходный массив
        let mut array = vec![
            Value::Number(1.0),
            Value::Number(2.0),
        ];

        // Добавляем новый элемент
        let result = push(&mut array, Value::Number(3.0));

        // ✅ Проверяем возвращаемое значение (новая длина)
        assert_eq!(result, Value::Number(3.0));

        // ✅ Проверяем, что массив действительно изменился
        assert_eq!(
            array,
            vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0),
            ]
        );
    }

    #[test]
    fn test_push_with_different_value_types() {
        let mut array = vec![
            Value::String("hello".to_string()),
            Value::Bool(true),
        ];

        let len = push(&mut array, Value::Number(42.0));

        // ✅ Новая длина — 3
        assert_eq!(len, Value::Number(3.0));

        // ✅ Массив хранит разные типы, как в JS
        assert_eq!(
            array,
            vec![
                Value::String("hello".to_string()),
                Value::Bool(true),
                Value::Number(42.0),
            ]
        );
    }

    #[test]
    fn test_push_multiple_times() {
        let mut array = vec![];
        let len1 = push(&mut array, Value::Number(1.0));
        let len2 = push(&mut array, Value::Number(2.0));
        let len3 = push(&mut array, Value::Number(3.0));

        assert_eq!(len1, Value::Number(1.0));
        assert_eq!(len2, Value::Number(2.0));
        assert_eq!(len3, Value::Number(3.0));

        assert_eq!(
            array,
            vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0),
            ]
        );
    }
}
