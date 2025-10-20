use std::io::{self, Write};

use crate::value::Value;

pub fn log(args: Vec<String>) {
    let mut stdout = io::stdout();
    log_with_writer(args, &mut stdout);
}

pub fn log_with_writer<W>(args: Vec<String>, writer: &mut W)
where
    W: Write,
{
    match format_log_message(args) {
        Some(message) => {
            let _ = writeln!(writer, "{}", message);
        }
        None => {
            let _ = writeln!(writer);
        }
    }
}

fn format_log_message(args: Vec<String>) -> Option<String> {
    let mut iter = args.into_iter();
    let first = iter.next()?;

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

        Some(formatted)
    } else {
        let mut parts = Vec::new();
        parts.push(first);
        parts.extend(iter);
        Some(parts.join(" "))
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

impl<T, const N: usize> ConsoleArg for [T; N]
where
    T: ConsoleArg,
{
    fn to_value(&self) -> Value {
        let elements = self.iter().map(|v| v.to_value()).collect::<Vec<_>>();
        Value::Array(elements)
    }
}

impl<T> ConsoleArg for Vec<T>
where
    T: ConsoleArg,
{
    fn to_value(&self) -> Value {
        let elements = self.iter().map(|v| v.to_value()).collect::<Vec<_>>();
        Value::Array(elements)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn capture_log(args: Vec<&str>) -> String {
        let mut buffer = Vec::new();
        log_with_writer(
            args.into_iter().map(|s| s.to_string()).collect(),
            &mut buffer,
        );
        String::from_utf8(buffer).expect("stdout must stay valid UTF-8")
    }

    #[test]
    fn log_prints_empty_line_when_no_arguments() {
        let output = capture_log(Vec::new());
        assert_eq!(output, "\n");
    }

    #[test]
    fn log_prints_simple_space_separated_arguments() {
        let output = capture_log(vec!["hello", "world", "42"]);
        assert_eq!(output, "hello world 42\n");
    }

    #[test]
    fn log_interpolates_placeholders_with_arguments() {
        let output = capture_log(vec!["value: ${}", "42"]);
        assert_eq!(output, "value: 42\n");
    }

    #[test]
    fn log_leaves_placeholder_when_argument_missing() {
        let output = capture_log(vec!["a ${} b ${}", "X"]);
        assert_eq!(output, "a X b ${}\n");
    }

    #[test]
    fn log_appends_extra_arguments_after_template() {
        let output = capture_log(vec!["x ${}", "1", "tail", "42"]);
        assert_eq!(output, "x 1 tail 42\n");
    }

    #[test]
    fn log_preserves_literal_dollar_braces_without_placeholder() {
        let output = capture_log(vec!["price ${10}"]);
        assert_eq!(output, "price ${10}\n");
    }

    #[test]
    fn log_allows_empty_template_and_trailing_arguments() {
        let output = capture_log(vec!["${}${}", "a", "b", "c"]);
        assert_eq!(output, "ab c\n");
    }
}
