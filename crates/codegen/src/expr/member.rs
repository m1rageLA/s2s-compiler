use ir::IrExpression;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::Codegen;

pub(crate) fn member_tokens(object: &IrExpression, property: &str) -> TokenStream {
    let object_tokens = object.codegen();
    let property_ident = format_ident!("{}", property);
    quote! { (#object_tokens).#property_ident }
}

#[cfg(test)]
mod tests {
    use super::*;
    use prettyplease;
    use proc_macro2::TokenStream;
    use quote::quote;
    use syn::{self, Expr, ExprField, ExprParen, ExprPath, Member};

    fn parse_field(tokens: TokenStream) -> ExprField {
        match syn::parse2::<Expr>(tokens).expect("member expression should parse") {
            Expr::Field(field) => field,
            _ => panic!("expected field expression"),
        }
    }

    fn base_ident(field: &ExprField) -> String {
        match field.base.as_ref() {
            Expr::Paren(ExprParen { expr, .. }) => match expr.as_ref() {
                Expr::Path(ExprPath { path, .. }) => path
                    .get_ident()
                    .expect("base identifier")
                    .to_string(),
                _ => panic!("unexpected base expression"),
            },
            _ => panic!("expected parenthesized base expression"),
        }
    }

    fn member_ident(field: &ExprField) -> String {
        match &field.member {
            Member::Named(ident) => ident.to_string(),
            _ => panic!("unexpected member expression"),
        }
    }

    fn render_expr(tokens: TokenStream) -> String {
        let module = quote! {
            fn main() {
                let value = #tokens;
            }
        };
        let file: syn::File = syn::parse2(module).expect("wrapped module should parse");
        prettyplease::unparse(&file)
    }

    #[test]
    fn test_member_tokens() {
        let object = IrExpression::Identifier("object".to_string());
        let tokens = member_tokens(&object, "property");
        let field = parse_field(tokens.clone());

        assert_eq!(base_ident(&field), "object");
        assert_eq!(member_ident(&field), "property");

        let rendered = render_expr(tokens);
        assert!(
            rendered.contains("let value = (object).property;"),
            "formatted output:\n{rendered}"
        );
    }

    #[test]
    fn test_property_with_underscore() {
        let object = IrExpression::Identifier("obj".to_string());
        let tokens = member_tokens(&object, "long_name");
        let field = parse_field(tokens.clone());

        assert_eq!(base_ident(&field), "obj");
        assert_eq!(member_ident(&field), "long_name");

        let rendered = render_expr(tokens);
        assert!(
            rendered.contains("let value = (obj).long_name;"),
            "formatted output:\n{rendered}"
        );
    }

    #[test]
    fn test_parentheses_and_member_are_preserved() {
        let object = IrExpression::Identifier("data".to_string());
        let tokens = member_tokens(&object, "value");
        let field = parse_field(tokens.clone());

        assert_eq!(base_ident(&field), "data");
        assert_eq!(member_ident(&field), "value");

        let rendered = render_expr(tokens);
        assert!(
            rendered.contains("let value = (data).value;"),
            "formatted output:\n{rendered}"
        );
    }
}
