use proc_macro2::{Literal, TokenStream};
use quote::quote;

pub fn unsupported_stmt(reason: &str) -> TokenStream {
    let msg = Literal::string(&format!("unsupported statement: {reason}"));
    quote! { panic!(#msg) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embeds_reason_in_panic() {
        let tokens = unsupported_stmt("test reason");
        assert_eq!(
            tokens.to_string(),
            quote::quote! { panic!("unsupported statement: test reason") }.to_string()
        );
    }
}
