use std::{unreachable};

use proc_macro2::TokenStream;
use quote::quote;

use crate::transform::{identifier, patterns, statements};



pub fn emit(fn_decl: swc_ecma_ast::FnDecl) -> TokenStream {
    let ident = identifier::identifier(fn_decl.ident.clone());
    let params = fn_decl.function.params.iter().map(|f| patterns::emit(f.pat.clone()));
    let body = match fn_decl.function.body.clone() {
        Some(block) => statements::block::emit(block),
        None => quote! {},
    };
    match fn_decl.function.body {
        Some(_) => quote! { fn #ident(#(#params),*) #body },
        None => todo!("emit function declaration without a body")
    }
}
