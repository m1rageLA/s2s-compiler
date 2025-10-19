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
