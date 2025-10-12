use crate::value::Value;

pub fn log(args: Vec<String>) {
    if args.is_empty() {
        println!();
        return;
    }

    let mut iter = args.into_iter();
    let first = iter.next().unwrap();

    if first.contains("${}") {
        let mut formatted = String::new();
        let mut cursor = first.as_str();

        while let Some(index) = cursor.find("${}") {
            let (head, tail) = cursor.split_at(index);
            formatted.push_str(head);

            if let Some(value) = iter.next() {
                formatted.push_str(&value);
            } else {
                formatted.push_str("${}");
            }

            cursor = &tail[3..];
        }

        formatted.push_str(cursor);

        let remaining: Vec<String> = iter.collect();
        if !remaining.is_empty() {
            if !formatted.is_empty() {
                formatted.push(' ');
            }
            formatted.push_str(&remaining.join(" "));
        }

        println!("{}", formatted);
    } else {
        let mut parts = Vec::new();
        parts.push(first);
        parts.extend(iter);
        println!("{}", parts.join(" "));
    }
}

pub fn stringify<T>(value: &T) -> String
where
    T: ConsoleArg + ?Sized,
{
    value.to_value().to_string()
}

pub trait ConsoleArg {
    fn to_value(&self) -> Value;
}

impl ConsoleArg for Value {
    fn to_value(&self) -> Value {
        self.clone()
    }
}

impl ConsoleArg for bool {
    fn to_value(&self) -> Value {
        Value::Bool(*self)
    }
}

impl ConsoleArg for i32 {
    fn to_value(&self) -> Value {
        Value::Number(*self as f64)
    }
}

impl ConsoleArg for i64 {
    fn to_value(&self) -> Value {
        Value::Number(*self as f64)
    }
}

impl ConsoleArg for i128 {
    fn to_value(&self) -> Value {
        Value::Number(*self as f64)
    }
}

impl ConsoleArg for f32 {
    fn to_value(&self) -> Value {
        Value::Number(*self as f64)
    }
}

impl ConsoleArg for f64 {
    fn to_value(&self) -> Value {
        Value::Number(*self)
    }
}

impl ConsoleArg for String {
    fn to_value(&self) -> Value {
        Value::String(self.clone())
    }
}

impl ConsoleArg for &str {
    fn to_value(&self) -> Value {
        Value::String((*self).to_string())
    }
}

impl ConsoleArg for () {
    fn to_value(&self) -> Value {
        Value::Undefined
    }
}
