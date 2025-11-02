use crate::Codegen;
use ir::IrExpression;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn join_tokens(target: &IrExpression, separator: Option<&IrExpression>) -> TokenStream {
    let target_tokens = target.codegen();
    let separator_tokens = match separator {
        Some(expr) => {
            let tokens = expr.codegen();
            quote! { Some(runtime::value::into_value((#tokens).clone())) }
        }
        None => quote! { None },
    };
    quote! { runtime::array::join(&#target_tokens, #separator_tokens) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ir::IrExpression;

    #[test]
    fn join_without_separator_defaults_to_none() {
        let tokens = join_tokens(&IrExpression::Identifier("values".into()), None);
        assert_eq!(
            tokens.to_string(),
            quote::quote! { runtime::array::join(&values, None) }.to_string()
        );
    }
}
