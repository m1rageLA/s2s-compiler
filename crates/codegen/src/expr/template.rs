use ir::IrTemplatePart;
use proc_macro2::{Literal, TokenStream};
use quote::quote;

use crate::Codegen;

pub(crate) fn template_literal_tokens(parts: &[IrTemplatePart]) -> TokenStream {
    let mut format_string = String::new();
    let mut expr_tokens: Vec<TokenStream> = Vec::new();

    for part in parts {
        match part {
            IrTemplatePart::String(text) => format_string.push_str(&escape_format_text(text)),
            IrTemplatePart::Expr(expr) => {
                format_string.push_str("{}");
                let inner = expr.codegen();
                expr_tokens.push(quote! { runtime::console::stringify(&(#inner)) });
            }
        }
    }

    let fmt_literal = Literal::string(&format_string);

    if expr_tokens.is_empty() {
        quote! { #fmt_literal.to_string() }
    } else {
        quote! { format!(#fmt_literal #(, #expr_tokens)*) }
    }
}

fn escape_format_text(text: &str) -> String {
    let mut escaped = String::with_capacity(text.len());
    for ch in text.chars() {
        match ch {
            '{' => escaped.push_str("{{"),
            '}' => escaped.push_str("}}"),
            _ => escaped.push(ch),
        }
    }
    escaped
}
