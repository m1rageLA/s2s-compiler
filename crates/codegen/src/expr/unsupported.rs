use proc_macro2::{Literal, TokenStream};
use quote::quote;

pub(crate) fn unsupported_expr(kind: &str) -> TokenStream {
    let msg = Literal::string(&format!("codegen for {kind} not implemented"));
    quote! { panic!(#msg) }
}

pub(crate) fn unsupported_bin_op(name: &str) -> TokenStream {
    let msg = Literal::string(&format!("codegen for binary op `{name}` not implemented"));
    quote! { panic!(#msg) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsupported_expr_panic_mentions_kind() {
        let tokens = unsupported_expr("super call");
        assert_eq!(
            tokens.to_string(),
            quote::quote! { panic!("codegen for super call not implemented") }.to_string()
        );
    }

    #[test]
    fn unsupported_binary_op_mentions_operator() {
        let tokens = unsupported_bin_op("instanceof");
        assert_eq!(
            tokens.to_string(),
            quote::quote! { panic!("codegen for binary op `instanceof` not implemented") }
                .to_string()
        );
    }
}
