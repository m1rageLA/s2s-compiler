use ir::IrExpression;
use proc_macro2::TokenStream;
use quote::quote;

use crate::Codegen;

pub(crate) fn call_tokens(callee: &IrExpression, args: &[IrExpression]) -> TokenStream {
    let callee_tokens = callee.codegen();
    let arg_tokens: Vec<TokenStream> = args
        .iter()
        .map(|arg| {
            let tokens = arg.codegen();
            quote! { (#tokens).clone() }
        })
        .collect();
    quote! { (#callee_tokens)( #( #arg_tokens ),* ) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ir::{IrExpression, IrLiteral};
    use syn::{Expr, ExprCall};

    fn parse_call(tokens: TokenStream) -> ExprCall {
        match syn::parse2::<Expr>(tokens).expect("call expression should parse") {
            Expr::Call(call) => call,
            _ => panic!("expected call expression"),
        }
    }

    #[test]
    fn call_tokens_wraps_callee_and_arguments() {
        let callee = IrExpression::Identifier("make_value".into());
        let args = vec![
            IrExpression::Literal(IrLiteral::Number(1.0)),
            IrExpression::Literal(IrLiteral::Bool(true)),
        ];

        let call = parse_call(call_tokens(&callee, &args));

        let callee_ident = match call.func.as_ref() {
            Expr::Paren(paren) => match paren.expr.as_ref() {
                Expr::Path(path) => path.path.get_ident().map(|ident| ident.to_string()),
                _ => None,
            },
            _ => None,
        }
        .expect("callee should be identifier");
        assert_eq!(callee_ident, "make_value");
        assert_eq!(call.args.len(), 2);
    }
}
