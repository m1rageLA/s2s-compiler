use proc_macro2::TokenStream;
use logger::unsupported;
use swc_ecma_ast::Pat;
use crate::transform::{identifier};
pub mod ident;

pub fn emit(pat: Pat) -> TokenStream {
    match pat {
        Pat::Ident(binding_ident) => identifier::identifier(binding_ident.id),
        _ => unsupported!(pat),
    }
}
