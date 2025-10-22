use super::ArrayLike;
use crate::value::Value;

pub fn length<T>(array: &T) -> Value
where
    T: ArrayLike + ?Sized,
{
    Value::Number(length_number(array))
}

pub fn length_number<T>(array: &T) -> f64
where
    T: ArrayLike + ?Sized,
{
    array.len() as f64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::Value;
    use std::any::Any;

    #[test]
    fn returns_length_of_array() {
        let array = vec![
            Value::Number(1.0),
            Value::String("hello".into()),
            Value::Bool(true),
        ];

        let result = length(&array);

        assert_eq!(result, Value::Number(3.0));
    }

    #[test]
    fn handles_empty_array() {
        let array: Vec<Value> = vec![];

        let result = length(&array);

        assert_eq!(result, Value::Number(0.0));
    }

    #[test]
    fn length_number_supports_boxed_arrays() {
        let boxed: Box<dyn Any> = Box::new(vec![Value::Number(10.0)]);

        let result = length_number(&boxed);

        assert!((result - 1.0).abs() < f64::EPSILON);
    }
}
