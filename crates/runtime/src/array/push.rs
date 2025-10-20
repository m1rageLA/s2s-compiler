pub fn push<T>(array: &mut Vec<T>, mut values: Vec<T>) -> f64 {
    array.append(&mut values);
    array.len() as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_adds_element_and_returns_new_length() {
        let mut array = vec![1.0, 2.0];

        let result = push(&mut array, vec![3.0]);

        assert_eq!(result, 3.0);
        assert_eq!(array, vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_push_with_different_value_types() {
        let mut array = vec!["hello".to_string(), "world".to_string()];

        let len = push(&mut array, vec!["rust".to_string()]);

        assert_eq!(len, 3.0);
        assert_eq!(
            array,
            vec![
                "hello".to_string(),
                "world".to_string(),
                "rust".to_string(),
            ]
        );
    }

    #[test]
    fn test_push_multiple_times() {
        let mut array = vec![];
        let len1 = push(&mut array, vec![1]);
        let len2 = push(&mut array, vec![2, 3]);

        assert_eq!(len1, 1.0);
        assert_eq!(len2, 3.0);
        assert_eq!(array, vec![1, 2, 3]);
    }
}
