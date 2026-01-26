use ir::{ArrayCall, IrArrayKind, IrExpression, IrType, RuntimeNamespace};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{typing, Codegen};

pub(crate) fn member_tokens(object: &IrExpression, property: &str) -> TokenStream {
    let object_tokens = object_tokens_for_member(object);
    let property_ident = format_ident!("{}", property);
    quote! { (#object_tokens).#property_ident }
}

fn object_tokens_for_member(object: &IrExpression) -> TokenStream {
    if let IrExpression::Identifier(name) = object {
        if let Some(alias) = typing::lookup_object_alias(name) {
            return object_index_tokens(&alias.target, &alias.index);
        }
    }

    if let IrExpression::RuntimeCall(RuntimeNamespace::Array(ArrayCall::Index { target, index, element })) = object {
        if matches!(element, Some(IrArrayKind::Object(_))) {
            return object_index_tokens(target.as_ref(), index.as_ref());
        }
    }

    object.codegen()
}

fn object_index_tokens(target: &IrExpression, index: &IrExpression) -> TokenStream {
    if let (IrExpression::Identifier(array_name), IrExpression::Identifier(index_name)) =
        (target, index)
    {
        if let Some(alias) = typing::lookup_array_index_alias(array_name, index_name) {
            let alias_ident = format_ident!("{}", alias);
            return quote! { *#alias_ident };
        }
    }

    let target_tokens = target.codegen();
    let index_tokens = index.codegen();
    match typing::infer_expression_type(index) {
        Some(IrType::UInt) => quote! { #target_tokens[#index_tokens] },
        _ => quote! { #target_tokens[(#index_tokens) as usize] },
    }
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
                Expr::Path(ExprPath { path, .. }) => {
                    path.get_ident().expect("base identifier").to_string()
                }
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
