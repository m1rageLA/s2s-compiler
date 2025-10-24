use std::collections::BTreeMap;

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

