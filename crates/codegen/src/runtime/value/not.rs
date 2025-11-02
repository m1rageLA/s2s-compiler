use crate::Codegen;
use ir::IrExpression;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn logical_not_tokens(expr: &IrExpression) -> TokenStream {
    let value_tokens = expr.codegen();
    quote! {{
        let value_tmp = runtime::value::into_value((#value_tokens).clone());
        runtime::value::ops::logical_not(value_tmp)
    }}
}

#[cfg(test)]
mod tests {
    use super::*;
    use ir::{IrExpression, IrLiteral};

    #[test]
    fn generates_logical_not_call() {
        let expr = IrExpression::Literal(IrLiteral::Number(0.0));
        let tokens = logical_not_tokens(&expr);
        assert_eq!(
            tokens.to_string(),
            quote! {{
                let value_tmp = runtime::value::into_value((runtime::value::Value::Number(0.0)).clone());
                runtime::value::ops::logical_not(value_tmp)
            }}
            .to_string()
        );
    }
}
