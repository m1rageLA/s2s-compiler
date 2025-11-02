use crate::value::Value;

pub fn length(target: Value) -> Value {
    let string = into_string(target);
    Value::Number(string.chars().count() as f64)
}

pub fn to_upper_case(target: Value) -> Value {
    let string = into_string(target);
    Value::String(string.to_uppercase())
}

pub fn to_lower_case(target: Value) -> Value {
    let string = into_string(target);
    Value::String(string.to_lowercase())
}

pub fn split(target: Value, separator: Option<Value>, limit: Option<Value>) -> Vec<Value> {
    let string = into_string(target);
    let limit = limit.map(|value| {
        let number = value.to_number();
        if number.is_nan() || number <= 0.0 {
            0
        } else {
            number.floor() as usize
        }
    });

    if limit == Some(0) {
        return Vec::new();
    }

    let separator = separator.and_then(|value| match value {
        Value::Undefined => None,
        other => Some(into_string(other)),
    });

    let mut parts: Vec<String> = match separator {
        None => vec![string.clone()],
        Some(ref sep) if sep.is_empty() => string.chars().map(|c| c.to_string()).collect(),
        Some(sep) => string.split(&sep).map(|s| s.to_string()).collect(),
    };

    if let Some(limit) = limit {
        if parts.len() > limit {
            parts.truncate(limit);
        }
    }

    parts.into_iter().map(Value::String).collect()
}

pub fn replace(target: Value, pattern: Value, replacement: Value) -> Value {
    let mut string = into_string(target);
    let pattern = into_string(pattern);
    let replacement = into_string(replacement);

    if pattern.is_empty() {
        let mut result = String::new();
        result.push_str(&replacement);
        result.push_str(&string);
        return Value::String(result);
    }

    if let Some(pos) = string.find(&pattern) {
        string.replace_range(pos..pos + pattern.len(), &replacement);
    }

    Value::String(string)
}

pub fn includes(target: Value, search: Value, position: Option<Value>) -> bool {
    let string = into_string(target);
    let search = into_string(search);
    let len = string.chars().count();
    let start = position
        .map(|value| {
            let number = value.to_number();
            if number.is_nan() {
                0
            } else if number <= 0.0 {
                0
            } else {
                number.floor() as usize
            }
        })
        .unwrap_or(0)
        .min(len);

    if search.is_empty() {
        return true;
    }

    let suffix: String = string.chars().skip(start).collect();
    suffix.contains(&search)
}

pub fn concat(target: Value, args: Vec<Value>) -> Value {
    let mut result = into_string(target);
    for arg in args {
        result.push_str(&into_string(arg));
    }
    Value::String(result)
}

pub fn slice(target: Value, start: Option<Value>, end: Option<Value>) -> Value {
    let string = into_string(target);
    let chars: Vec<char> = string.chars().collect();
    let len = chars.len() as isize;

    let start = normalize_slice_index(start, len, 0);
    let end = match end {
        Some(value) => normalize_slice_index(Some(value), len, len),
        None => len,
    };

    let start = clamp_index(start, len);
    let end = clamp_index(end, len);
    let (start, end) = if end < start {
        (start, start)
    } else {
        (start, end)
    };

    Value::String(chars[start..end].iter().collect())
}

pub fn substr(target: Value, start: Option<Value>, length: Option<Value>) -> Value {
    let string = into_string(target);
    let chars: Vec<char> = string.chars().collect();
    let len = chars.len() as isize;

    let mut start_index = match start {
        Some(value) => {
            let number = to_isize(value);
            if number < 0 { len + number } else { number }
        }
        None => 0,
    };

    if start_index < 0 {
        start_index = 0;
    }
    if start_index > len {
        start_index = len;
    }

    let length = length.map_or(len - start_index, |value| {
        let number = to_isize(value);
        if number < 0 { 0 } else { number }
    });

    let end_index = (start_index + length).min(len).max(start_index);
    let start = start_index as usize;
    let end = end_index as usize;

    Value::String(chars[start..end].iter().collect())
}

fn into_string(value: Value) -> String {
    match value {
        Value::String(s) => s,
        other => other.to_string(),
    }
}

fn normalize_slice_index(value: Option<Value>, len: isize, default: isize) -> isize {
    value.map_or(default, |v| {
        let index = to_isize(v);
        if index < 0 { len + index } else { index }
    })
}

fn clamp_index(index: isize, len: isize) -> usize {
    if index <= 0 {
        0
    } else if index >= len {
        len as usize
    } else {
        index as usize
    }
}

fn to_isize(value: Value) -> isize {
    let number = value.to_number();
    if number.is_nan() || number.is_infinite() {
        0
    } else if number >= 0.0 {
        number.floor() as isize
    } else {
        number.ceil() as isize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::Value;

    fn string(value: &str) -> Value {
        Value::String(value.to_string())
    }

    fn number(value: f64) -> Value {
        Value::Number(value)
    }

    #[test]
    fn length_counts_characters() {
        assert_eq!(length(string("hello")), Value::Number(5.0));
        assert_eq!(length(string("")), Value::Number(0.0));
    }

    #[test]
    fn to_upper_case_converts_letters() {
        assert_eq!(to_upper_case(string("Hello World")), string("HELLO WORLD"));
        assert_eq!(to_upper_case(string("héllo")), string("HÉLLO"));
    }

    #[test]
    fn to_lower_case_converts_letters() {
        assert_eq!(to_lower_case(string("Hello WORLD")), string("hello world"));
        assert_eq!(to_lower_case(string("HÉLLO")), string("héllo"));
    }

    #[test]
    fn split_without_separator_returns_original() {
        assert_eq!(split(string("value"), None, None), vec![string("value")]);
        assert_eq!(
            split(string("value"), Some(Value::Undefined), None),
            vec![string("value")]
        );
    }

    #[test]
    fn split_with_separator_and_limit() {
        assert_eq!(
            split(string("a,b,c"), Some(string(",")), Some(number(2.0))),
            vec![string("a"), string("b")]
        );
    }

    #[test]
    fn split_with_zero_limit_returns_empty() {
        assert_eq!(
            split(string("a,b,c"), Some(string(",")), Some(number(0.0))),
            Vec::<Value>::new()
        );
    }

    #[test]
    fn split_with_empty_separator_breaks_into_chars() {
        assert_eq!(
            split(string("abc"), Some(string("")), None),
            vec![string("a"), string("b"), string("c")]
        );
    }

    #[test]
    fn replace_first_occurrence_of_pattern() {
        assert_eq!(
            replace(string("hello world"), string("world"), string("rust")),
            string("hello rust")
        );
    }

    #[test]
    fn replace_with_missing_pattern_returns_original() {
        assert_eq!(
            replace(string("hello"), string("x"), string("y")),
            string("hello")
        );
    }

    #[test]
    fn includes_honors_start_position() {
        assert!(includes(
            string("hello world"),
            string("world"),
            Some(number(6.0))
        ));
        assert!(!includes(
            string("hello world"),
            string("hello"),
            Some(number(6.0))
        ));
    }

    #[test]
    fn includes_with_empty_search_always_true() {
        assert!(includes(string("abc"), string(""), None));
    }

    #[test]
    fn concat_combines_all_arguments() {
        assert_eq!(
            concat(
                string("Hello"),
                vec![string(", "), string("world"), number(1.0)]
            ),
            string("Hello, world1")
        );
    }

    #[test]
    fn slice_supports_positive_and_negative_indices() {
        assert_eq!(
            slice(string("abcdef"), Some(number(2.0)), Some(number(5.0))),
            string("cde")
        );
        assert_eq!(
            slice(string("abcdef"), Some(number(-3.0)), None),
            string("def")
        );
    }

    #[test]
    fn slice_with_end_before_start_returns_empty() {
        assert_eq!(
            slice(string("abcdef"), Some(number(4.0)), Some(number(2.0))),
            string("")
        );
    }

    #[test]
    fn substr_supports_negative_start_and_length() {
        assert_eq!(
            substr(string("abcdef"), Some(number(2.0)), Some(number(3.0))),
            string("cde")
        );
        assert_eq!(
            substr(string("abcdef"), Some(number(-2.0)), Some(number(2.0))),
            string("ef")
        );
    }

    #[test]
    fn substr_without_length_extends_to_end() {
        assert_eq!(
            substr(string("typescript"), Some(number(4.0)), None),
            string("script")
        );
    }
}
