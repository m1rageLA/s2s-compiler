use proc_macro2::Span;
use syn::Lifetime;

pub(crate) fn sanitize_label(label: &str) -> String {
    let mut clean = String::new();
    for ch in label.chars() {
        if clean.is_empty() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                clean.push(ch);
            } else if ch.is_ascii_digit() {
                clean.push('_');
                clean.push(ch);
            } else {
                clean.push('_');
            }
        } else if ch.is_ascii_alphanumeric() || ch == '_' {
            clean.push(ch);
        } else {
            clean.push('_');
        }
    }

    let base = if clean.is_empty() { "_label".into() } else { clean };
    format!("ts_label_{base}")
}

pub(crate) fn label_lifetime(label: &str) -> Lifetime {
    let sanitized = sanitize_label(label);
    Lifetime::new(&format!("'{}", sanitized), Span::mixed_site())
}
