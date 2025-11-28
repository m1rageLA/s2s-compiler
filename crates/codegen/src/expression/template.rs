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
        assert_eq!(tokens.to_string(), quote::quote! { "plain".to_string() }.to_string());
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
            Expr::Macro(mac) => {
                assert_eq!(mac.mac.path.segments[0].ident.to_string(), "format");

                let parser = syn::punctuated::Punctuated::<Expr, syn::Token![,]>::parse_terminated;
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
            _ => panic!("unexpected expression"),
        }
    }
}
