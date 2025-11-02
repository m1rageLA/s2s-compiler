use super::types::Value;

pub fn add<L, R>(lhs: L, rhs: R) -> Value
where
    L: Into<Value>,
    R: Into<Value>,
{
    let left = lhs.into();
    let right = rhs.into();
    if left.is_string_like() || right.is_string_like() {
        Value::String(format!("{}{}", left.to_string(), right.to_string()))
    } else {
        Value::Number(left.to_number() + right.to_number())
    }
}

/// Subtracts two values. If either value is a string, it will be
/// treated as a number (i.e. "5" will be treated as 5.0). If both values
/// are strings, it will concatenate them. If one value is a string and
/// the other is a number, it will treat the string as a number (i.e. "5"
/// will be treated as 5.0).
pub fn sub<L, R>(lhs: L, rhs: R) -> Value
where
    L: Into<Value>,
    R: Into<Value>,
{
    Value::Number(sub_number(lhs, rhs))
}

pub fn mul<L, R>(lhs: L, rhs: R) -> Value
where
    L: Into<Value>,
    R: Into<Value>,
{
    Value::Number(lhs.into().to_number() * rhs.into().to_number())
}

pub fn div<L, R>(lhs: L, rhs: R) -> Value
where
    L: Into<Value>,
    R: Into<Value>,
{
    Value::Number(div_number(lhs, rhs))
}

pub fn modulo<L, R>(lhs: L, rhs: R) -> Value
where
    L: Into<Value>,
    R: Into<Value>,
{
    Value::Number(mod_number(lhs, rhs))
}

pub fn loose_equal<L, R>(lhs: L, rhs: R) -> bool
where
    L: Into<Value>,
    R: Into<Value>,
{
    let left = lhs.into();
    let right = rhs.into();
    loose_equal_refs(&left, &right)
}

pub fn strict_equal<L, R>(lhs: L, rhs: R) -> bool
where
    L: Into<Value>,
    R: Into<Value>,
{
    let left = lhs.into();
    let right = rhs.into();
    strict_equal_refs(&left, &right)
}

pub fn loose_not_equal<L, R>(lhs: L, rhs: R) -> bool
where
    L: Into<Value>,
    R: Into<Value>,
{
    let left = lhs.into();
    let right = rhs.into();
    loose_not_equal_refs(&left, &right)
}

pub fn strict_not_equal<L, R>(lhs: L, rhs: R) -> bool
where
    L: Into<Value>,
    R: Into<Value>,
{
    let left = lhs.into();
    let right = rhs.into();
    strict_not_equal_refs(&left, &right)
}

pub fn loose_equal_refs(left: &Value, right: &Value) -> bool {
    loose_equal_values(left, right)
}

pub fn strict_equal_refs(left: &Value, right: &Value) -> bool {
    left == right
}

pub fn loose_not_equal_refs(left: &Value, right: &Value) -> bool {
    !loose_equal_refs(left, right)
}

pub fn strict_not_equal_refs(left: &Value, right: &Value) -> bool {
    !strict_equal_refs(left, right)
}

pub fn logical_not<V>(value: V) -> bool
where
    V: Into<Value>,
{
    !value.into().to_boolean()
}

pub fn get_property<V>(value: V, property: &str) -> Value
where
    V: Into<Value>,
{
    match value.into() {
        Value::Object(map) => map.get(property).cloned().unwrap_or(Value::Undefined),
        Value::Array(values) => match property {
            "length" => Value::Number(values.len() as f64),
            _ => Value::Undefined,
        },
        Value::String(text) => match property {
            "length" => Value::Number(text.chars().count() as f64),
            _ => Value::Undefined,
        },
        Value::Number(_) | Value::Bool(_) | Value::Null | Value::Undefined => Value::Undefined,
    }
}

pub fn less_than<L, R>(lhs: L, rhs: R) -> bool
where
    L: Into<Value>,
    R: Into<Value>,
{
    compare_order(&lhs.into(), &rhs.into(), Ordering::Less)
}

pub fn less_than_or_equal<L, R>(lhs: L, rhs: R) -> bool
where
    L: Into<Value>,
    R: Into<Value>,
{
    compare_order(&lhs.into(), &rhs.into(), Ordering::LessOrEqual)
}

pub fn greater_than<L, R>(lhs: L, rhs: R) -> bool
where
    L: Into<Value>,
    R: Into<Value>,
{
    compare_order(&lhs.into(), &rhs.into(), Ordering::Greater)
}

pub fn greater_than_or_equal<L, R>(lhs: L, rhs: R) -> bool
where
    L: Into<Value>,
    R: Into<Value>,
{
    compare_order(&lhs.into(), &rhs.into(), Ordering::GreaterOrEqual)
}

pub fn sub_number<L, R>(lhs: L, rhs: R) -> f64
where
    L: Into<Value>,
    R: Into<Value>,
{
    let left: Value = lhs.into();
    let right: Value = rhs.into();
    left.to_number() - right.to_number()
}

pub fn mul_number<L, R>(lhs: L, rhs: R) -> f64
where
    L: Into<Value>,
    R: Into<Value>,
{
    let left: Value = lhs.into();
    let right: Value = rhs.into();
    left.to_number() * right.to_number()
}

pub fn div_number<L, R>(lhs: L, rhs: R) -> f64
where
    L: Into<Value>,
    R: Into<Value>,
{
    let left: Value = lhs.into();
    let right: Value = rhs.into();
    left.to_number() / right.to_number()
}

pub fn mod_number<L, R>(lhs: L, rhs: R) -> f64
where
    L: Into<Value>,
    R: Into<Value>,
{
    let left: Value = lhs.into();
    let right: Value = rhs.into();
    left.to_number() % right.to_number()
}

#[derive(Copy, Clone)]
enum Ordering {
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
}

fn loose_equal_values(left: &Value, right: &Value) -> bool {
    use Value::*;

    match (left, right) {
        (Null, Undefined) | (Undefined, Null) => true,
        (Number(a), Number(b)) => numbers_equal(*a, *b),
        (String(a), String(b)) => a == b,
        (Bool(a), Bool(b)) => a == b,
        _ => numbers_equal(left.to_number(), right.to_number()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logical_not_applies_js_truthiness() {
        assert!(logical_not(Value::Number(0.0)));
        assert!(logical_not(Value::String("".into())));
        assert!(logical_not(Value::Bool(false)));
        assert!(logical_not(Value::Undefined));
        assert!(logical_not(Value::Null));

        assert!(!logical_not(Value::Number(1.0)));
        assert!(!logical_not(Value::String("hi".into())));
        assert!(!logical_not(Value::Bool(true)));
        assert!(!logical_not(Value::Array(vec![])));
    }

    #[test]
    fn get_property_reads_object_fields() {
        let mut map = std::collections::BTreeMap::new();
        map.insert("name".into(), Value::String("Alice".into()));
        assert_eq!(
            get_property(Value::Object(map.clone()), "name"),
            Value::String("Alice".into())
        );
        assert_eq!(
            get_property(Value::Object(map), "missing"),
            Value::Undefined
        );
    }

    #[test]
    fn get_property_handles_length_for_collections() {
        let array = Value::Array(vec![Value::Number(1.0), Value::Number(2.0)]);
        assert_eq!(get_property(array, "length"), Value::Number(2.0));

        let text = Value::String("hi".into());
        assert_eq!(get_property(text, "length"), Value::Number(2.0));

        assert_eq!(get_property(Value::Number(1.0), "length"), Value::Undefined);
    }
}

fn numbers_equal(a: f64, b: f64) -> bool {
    if a.is_nan() || b.is_nan() {
        false
    } else {
        (a - b).abs() < f64::EPSILON
    }
}

fn compare_order(left: &Value, right: &Value, ordering: Ordering) -> bool {
    use Value::String;

    if let (String(a), String(b)) = (left, right) {
        return match ordering {
            Ordering::Less => a < b,
            Ordering::LessOrEqual => a <= b,
            Ordering::Greater => a > b,
            Ordering::GreaterOrEqual => a >= b,
        };
    }

    let l = left.to_number();
    let r = right.to_number();

    match ordering {
        Ordering::Less => l < r,
        Ordering::LessOrEqual => l <= r,
        Ordering::Greater => l > r,
        Ordering::GreaterOrEqual => l >= r,
    }
}
