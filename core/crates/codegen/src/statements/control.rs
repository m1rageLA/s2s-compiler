use proc_macro2::TokenStream;
use quote::quote;

use super::label::label_lifetime;

pub(crate) fn break_tokens(label: Option<&str>) -> TokenStream {
    match label {
        Some(name) => {
            let lifetime = label_lifetime(name);
            quote! { break #lifetime; }
        }
        None => quote! { break; },
    }
}

pub(crate) fn continue_tokens(label: Option<&str>) -> TokenStream {
    match label {
        Some(name) => {
            let lifetime = label_lifetime(name);
            quote! { continue #lifetime; }
        }
        None => quote! { continue; },
    }
}
