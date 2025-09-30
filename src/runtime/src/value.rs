pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Null,
    Undefined,
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Value::Number(n) => n.to_string(),
            Value::String(s) => s.clone(),
            Value::Bool(b) => b.to_string(),
            JsValue::Null => "null".into(),
            JsValue::Undefined => "undefined".into(),
        }
    }
}
