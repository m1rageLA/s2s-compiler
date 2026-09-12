use proc_macro2::TokenStream;
use swc_ecma_ast::Stmt;

use super::declarations;

pub fn emit(stmt: Stmt) -> TokenStream {
    match stmt {
        Stmt::Decl(decl) => declarations::emit(decl),
        _ => TokenStream::new(), // Handle other statements as needed
    }
}
