use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub(crate) fn identifier_tokens(name: &str) -> TokenStream {
    let ident = format_ident!("{}", name);
    quote! { #ident }
}
