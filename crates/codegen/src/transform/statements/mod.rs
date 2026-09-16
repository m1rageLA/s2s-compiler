use proc_macro2::TokenStream;
use logger::unsupported;
use swc_ecma_ast::Stmt;
pub mod block;
pub mod forstmt;
pub mod empty;

use super::declarations;

pub fn emit(stmt: Stmt) -> TokenStream {
    match stmt {
        Stmt::Decl(decl) => declarations::emit(decl),
        Stmt::Block(block) => block::emit(block),
        Stmt::For(for_stmt) => forstmt::emit(for_stmt),
        Stmt::Empty(_) => empty::emit(),
        _ => unsupported!(stmt),
    }
}
