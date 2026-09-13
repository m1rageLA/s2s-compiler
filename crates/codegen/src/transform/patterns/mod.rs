use proc_macro2::TokenStream;
use swc_ecma_ast::Pat;
use crate::transform::{identifier};
pub mod ident;

pub fn emit(pat: Pat) -> TokenStream {
    match pat {
        Pat::Ident(binding_ident) => identifier::identifier(binding_ident.id),
        _ => todo!("emit other patterns as needed"),
    }
}   