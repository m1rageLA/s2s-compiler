use proc_macro2::TokenStream;
use swc_ecma_ast::Stmt;
pub mod block;
use super::declarations;

pub fn emit(stmt: Stmt) -> TokenStream {
    match stmt {
        Stmt::Decl(decl) => declarations::emit(decl),
        Stmt::Block(block) => block::emit(block),
        _ => TokenStream::new(), // Handle other statements as needed
    }
}
