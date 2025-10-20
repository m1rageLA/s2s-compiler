#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Null,
    Undefined,
    //array
    Array(Vec<Value>),
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
        }
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
