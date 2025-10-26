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
        quote! { runtime::value::Value::String(#fmt_literal.to_string()) }
    } else {
        quote! { runtime::value::Value::String(format!(#fmt_literal #(, #expr_tokens)*)) }
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

#[cfg(test)]
mod tests {
    use super::*;
    use ir::{IrExpression, IrLiteral, IrTemplatePart};
    use quote::ToTokens;
    use syn::{Expr, parse::Parser};

    #[test]
    fn template_without_expressions_returns_string_literal() {
        let parts = vec![IrTemplatePart::String("plain".into())];
        let tokens = template_literal_tokens(&parts);
        let s = tokens.to_string();
        assert!(s.contains("runtime :: value :: Value :: String"));
        assert!(s.contains("to_string"));
    }

    #[test]
    fn template_with_expressions_uses_format_macro() {
        let parts = vec![
            IrTemplatePart::String("count: {".into()),
            IrTemplatePart::Expr(Box::new(IrExpression::Identifier("value".into()))),
            IrTemplatePart::String("}; total: ".into()),
            IrTemplatePart::Expr(Box::new(IrExpression::Literal(IrLiteral::Number(2.0)))),
        ];

        let tokens = template_literal_tokens(&parts);
        match syn::parse2::<Expr>(tokens).expect("format macro should parse") {
            Expr::Call(call) => {
                // The top-level call should be runtime::value::Value::String(format!(...))
                let callee_tokens = call.func.to_token_stream().to_string();
                assert!(callee_tokens.contains("runtime :: value :: Value :: String"));
                // First argument to the call should be a macro invocation (format!)
                let first_arg = call.args.iter().next().expect("expected arg");
                match first_arg {
                    Expr::Macro(mac) => {
                        assert_eq!(mac.mac.path.segments[0].ident.to_string(), "format");
                        let parser =
                            syn::punctuated::Punctuated::<Expr, syn::Token![,]>::parse_terminated;
                        let args = parser
                            .parse2(mac.mac.tokens.clone())
                            .expect("format macro arguments should parse");
                        let mut elems = args.iter();
                        let format_literal = match elems.next().expect("format string literal") {
                            Expr::Lit(lit) => match &lit.lit {
                                syn::Lit::Str(lit_str) => lit_str.value(),
                                _ => panic!("expected string literal for format!"),
                            },
                            _ => panic!("expected literal expression"),
                        };
                        assert_eq!(format_literal, "count: {{{}}}; total: {}");

                        for expr in elems {
                            let tokens = expr.to_token_stream().to_string();
                            assert!(
                                tokens.contains("runtime :: console :: stringify"),
                                "unexpected argument tokens: {tokens}"
                            );
                        }
                    }
                    other => panic!(
                        "expected macro as first arg, got {}",
                        other.to_token_stream().to_string()
                    ),
                }
            }
            _ => panic!("unexpected expression"),
        }
    }
}
