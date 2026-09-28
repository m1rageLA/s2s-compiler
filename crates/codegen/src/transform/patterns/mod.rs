use proc_macro2::TokenStream;
use logger::unsupported;
use swc_ecma_ast::Pat;
use crate::transform::{common};
pub mod ident;

pub fn emit(pat: Pat) -> TokenStream {
    match pat {
        Pat::Ident(binding_ident) => common::emit(binding_ident.id),
        _ => unsupported!(pat),
    }
}
