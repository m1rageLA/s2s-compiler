use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

/// Describes the shape of a [`Value`] without consuming it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueKind {
    Number,
    String,
    Bool,
    Null,
    Undefined,
    Array,
    Object,
}

impl fmt::Display for ValueKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match self {
            ValueKind::Number => "number",
            ValueKind::String => "string",
            ValueKind::Bool => "bool",
            ValueKind::Null => "null",
            ValueKind::Undefined => "undefined",
            ValueKind::Array => "array",
            ValueKind::Object => "object",
        };
        f.write_str(repr)
    }
}

/// Error raised when a [`Value`] cannot be converted into the requested Rust type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValueCastError {
    expected: &'static str,
    actual: ValueKind,
}

impl ValueCastError {
    pub fn new(expected: &'static str, actual: ValueKind) -> Self {
        Self { expected, actual }
    }

    pub fn expected(&self) -> &'static str {
        self.expected
    }

    pub fn actual(&self) -> ValueKind {
        self.actual
    }
}

impl fmt::Display for ValueCastError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "expected value of type `{}`, got `{}` instead",
            self.expected, self.actual
        )
    }
}

impl Error for ValueCastError {}

pub type ValueResult<T> = Result<T, ValueCastError>;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Null,
    Undefined,
    //array
    Array(Vec<Value>),
    Object(BTreeMap<String, Value>),
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Value::Number(n) => n.to_string(),
            Value::String(s) => s.clone(),
            Value::Bool(b) => b.to_string(),
            Value::Null => "null".into(),
            Value::Undefined => "undefined".into(),
            Value::Array(a) => a.iter().map(|v| v.to_string()).collect(),
            Value::Object(map) => {
                let mut parts = Vec::new();
                for (key, value) in map.iter() {
                    parts.push(format!("{key}: {}", value.to_string()));
                }
                format!("{{{}}}", parts.join(", "))
            }
        }
    }
}

impl Value {
    /// Returns the [`ValueKind`] describing this variant.
    pub fn kind(&self) -> ValueKind {
        match self {
            Value::Number(_) => ValueKind::Number,
            Value::String(_) => ValueKind::String,
            Value::Bool(_) => ValueKind::Bool,
            Value::Null => ValueKind::Null,
            Value::Undefined => ValueKind::Undefined,
            Value::Array(_) => ValueKind::Array,
            Value::Object(_) => ValueKind::Object,
        }
    }

    /// Fallible borrow-based conversion into `f64`.
    pub fn as_number(&self) -> ValueResult<f64> {
        match self {
            Value::Number(value) => Ok(*value),
            _ => Err(ValueCastError::new("number", self.kind())),
        }
    }

    /// Fallible owned conversion into `f64`.
    pub fn try_into_number(self) -> ValueResult<f64> {
        match self {
            Value::Number(value) => Ok(value),
            value => Err(ValueCastError::new("number", value.kind())),
        }
    }

    /// Owned conversion into `f64` that panics on type mismatch.
    pub fn into_number(self) -> f64 {
        self.try_into_number()
            .expect("expected runtime::value::Value::Number")
    }

    pub(crate) fn to_number(&self) -> f64 {
        match self {
            Value::Number(n) => *n,
            Value::Bool(true) => 1.0,
            Value::Bool(false) => 0.0,
            Value::Null => 0.0,
            Value::Undefined => f64::NAN,
            Value::String(s) => s.parse::<f64>().unwrap_or(f64::NAN),
            Value::Array(elements) if elements.is_empty() => 0.0,
            Value::Array(_) | Value::Object(_) => f64::NAN,
        }
    }

    pub(crate) fn to_boolean(&self) -> bool {
        match self {
            Value::Bool(value) => *value,
            Value::Null | Value::Undefined => false,
            Value::Number(value) => *value != 0.0 && !value.is_nan(),
            Value::String(value) => !value.is_empty(),
            Value::Array(_) | Value::Object(_) => true,
        }
    }

    pub(crate) fn is_string_like(&self) -> bool {
        matches!(self, Value::String(_) | Value::Array(_) | Value::Object(_))
    }
}

impl From<&Value> for Value {
    fn from(value: &Value) -> Self {
        value.clone()
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Number(value)
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Value::Number(value as f64)
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::Number(value as f64)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Number(value as f64)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::String(value.to_string())
    }
}

impl From<Vec<Value>> for Value {
    fn from(value: Vec<Value>) -> Self {
        Value::Array(value)
    }
}

impl From<BTreeMap<String, Value>> for Value {
    fn from(value: BTreeMap<String, Value>) -> Self {
        Value::Object(value)
    }
}

impl From<()> for Value {
    fn from(_: ()) -> Self {
        Value::Undefined
    }
}

pub fn into_value<T>(value: T) -> Value
where
    T: Into<Value>,
{
    value.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kind_reflects_value_variant() {
        assert_eq!(Value::Number(1.23).kind(), ValueKind::Number);
        assert_eq!(Value::String("hi".into()).kind(), ValueKind::String);
        assert_eq!(Value::Bool(true).kind(), ValueKind::Bool);
        assert_eq!(Value::Null.kind(), ValueKind::Null);
    }

    #[test]
    fn as_number_extracts_inner_value() {
        let number = Value::Number(4.2);
        assert_eq!(number.as_number().unwrap(), 4.2);
    }

    #[test]
    fn as_number_reports_type_mismatch() {
        let string = Value::String("nope".into());
        let err = string.as_number().unwrap_err();
        assert_eq!(err.expected(), "number");
        assert_eq!(err.actual(), ValueKind::String);
    }

    #[test]
    fn try_into_number_consumes_value() {
        let number = Value::Number(9.1);
        assert_eq!(number.try_into_number().unwrap(), 9.1);
    }

    #[test]
    #[should_panic(expected = "runtime::value::Value::Number")]
    fn into_number_panics_on_wrong_type() {
        let _ = Value::Bool(false).into_number();
    }
}
