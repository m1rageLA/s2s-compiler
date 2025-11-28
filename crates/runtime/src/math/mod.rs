use crate::value::Value;

/// Mirrors `Math.random()` returning a floating point in the range [0, 1).
pub fn random() -> Value {
    Value::Number(random_number())
}

pub fn random_number() -> f64 {
    rand::random::<f64>()
}

pub fn sqrt<V>(value: V) -> Value
where
    V: Into<Value>,
{
    let num = value.into().to_number();
    Value::Number(num.sqrt())
}

pub fn sqrt_number(value: f64) -> f64 {
    value.sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_returns_number_value_in_unit_interval() {
        match random() {
            Value::Number(value) => {
                assert!(value >= 0.0);
                assert!(value < 1.0);
            }
            other => panic!("expected number value from Math.random, got {other:?}"),
        }
    }

    #[test]
    fn random_number_stays_within_bounds() {
        let value = random_number();
        assert!(value >= 0.0);
        assert!(value < 1.0);
    }

    #[test]
    fn sqrt_operates_on_value_and_number() {
        let v = sqrt(Value::Number(9.0));
        assert_eq!(v, Value::Number(3.0));

        let n = sqrt_number(16.0);
        assert_eq!(n, 4.0);
    }
}
