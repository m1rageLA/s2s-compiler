use ir::{IrExpression, IrStmt};
use proc_macro2::TokenStream;
use quote::quote;

use super::collect_stmt_tokens;
use super::label::label_lifetime;
use crate::Codegen;

pub fn while_tokens(condition: &IrExpression, body: &[IrStmt], label: Option<&str>) -> TokenStream {
    let condition_tokens = condition.codegen();
    let body_tokens = collect_stmt_tokens(body);

    match label {
        Some(name) => {
            let lifetime = label_lifetime(name);
            quote! { #lifetime: while #condition_tokens { #(#body_tokens)* } }
        }
        None => quote! { while #condition_tokens { #(#body_tokens)* } },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ir::{IrExpression, IrLiteral, IrStmt};

    #[test]
    fn emits_while_loop_structure() {
        let condition = IrExpression::Literal(IrLiteral::Bool(true));
        let body = vec![IrStmt::Return(None)];

        let tokens = while_tokens(&condition, &body, None);
        let expected = quote! { while true { return (); } };

        assert_eq!(tokens.to_string(), expected.to_string());
    }
}
