use crate::Codegen;
use ir::IrExpression;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn array_tokens(items: &[IrExpression]) -> TokenStream {
    let item_tokens: Vec<TokenStream> = items.iter().map(|item| item.codegen()).collect();
    quote! { vec![ #( #item_tokens ),* ] }
}

#[cfg(test)]
mod tests {
    use super::*;
    use prettyplease;
    use proc_macro2::TokenStream;
    use quote::quote;
    use syn::{self, Expr};

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
    fn test_array_tokens() {
        let items = vec![
            IrExpression::RuntimeCall(ir::RuntimeNamespace::Value(ir::ValueCall::Coerce {
                expr: Box::new(IrExpression::Literal(ir::IrLiteral::Number(1.0))),
            })),
            IrExpression::RuntimeCall(ir::RuntimeNamespace::Value(ir::ValueCall::Coerce {
                expr: Box::new(IrExpression::Literal(ir::IrLiteral::Number(2.0))),
            })),
            IrExpression::RuntimeCall(ir::RuntimeNamespace::Value(ir::ValueCall::Coerce {
                expr: Box::new(IrExpression::Literal(ir::IrLiteral::Number(3.0))),
            })),
        ];
        let tokens = array_tokens(&items);

        let expr = syn::parse2::<Expr>(tokens.clone()).expect("array expression should parse");
        match expr {
            Expr::Macro(m) => {
                assert_eq!(m.mac.path.segments.last().unwrap().ident.to_string(), "vec");
            }
            _ => panic!("expected vec! macro expression"),
        }

        let rendered = render_expr(tokens);
        assert!(
            rendered.contains("runtime::value::Value::Number(1.0)"),
            "formatted output:\n{rendered}"
        );
        assert!(
            rendered.contains("runtime::value::Value::Number(2.0)"),
            "formatted output:\n{rendered}"
        );
        assert!(
            rendered.contains("runtime::value::Value::Number(3.0)"),
            "formatted output:\n{rendered}"
        );
    }
}
