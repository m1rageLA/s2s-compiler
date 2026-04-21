use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub(crate) fn identifier_tokens(name: &str) -> TokenStream {
    if name == "undefined" {
        return quote! { runtime::value::Value::Undefined };
    }
    let ident = format_ident!("{}", name);
    quote! { #ident }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifier_tokens_emits_identifier() {
        let tokens = identifier_tokens("value");
        assert_eq!(tokens.to_string(), quote::quote! { value }.to_string());
    }
}
