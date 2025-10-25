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
