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
