use ir::{IrExpression, IrStmt};
use proc_macro2::TokenStream;
use quote::quote;

use super::collect_stmt_tokens;
use super::label::label_lifetime;
use crate::Codegen;

pub fn do_while_tokens(
    body: &[IrStmt],
    condition: &IrExpression,
    label: Option<&str>,
) -> TokenStream {
    let body_tokens = collect_stmt_tokens(body);
    let condition_tokens = condition.codegen();

    match label {
        Some(name) => {
            let lifetime = label_lifetime(name);
            quote! {
                #lifetime: loop {
                    #(#body_tokens)*
                    if !(#condition_tokens) {
                        break #lifetime;
                    }
                }
            }
        }
        None => quote! {
            loop {
                #(#body_tokens)*
                if !(#condition_tokens) {
                    break;
                }
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ir::{IrExpression, IrLiteral, IrStmt};

    #[test]
    fn do_while_wraps_body_in_loop() {
        let body = vec![IrStmt::Return(None)];
        let condition = IrExpression::Literal(IrLiteral::Bool(true));

        let tokens = do_while_tokens(&body, &condition, None);
        let expected = quote! {
            loop {
                return ();
                if !(true) {
                    break;
                }
            }
        };

        assert_eq!(tokens.to_string(), expected.to_string());
    }
}
