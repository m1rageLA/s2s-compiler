use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

pub fn identifier(ident: swc_ecma_ast::Ident) -> TokenStream {
    let ident = Ident::new(ident.sym.as_ref(), Span::call_site());
    quote! { #ident }
}
