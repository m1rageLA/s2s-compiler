use crate::value::Value;

pub fn length(array: &Vec<Value>) -> Value {
    Value::Number(array.len() as f64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::Value;

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
}
