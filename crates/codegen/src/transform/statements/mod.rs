use std::todo;

use logger::unsupported;
use proc_macro2::TokenStream;
use swc_ecma_ast::Stmt;
pub mod block;
pub mod dowhile;
pub mod empty;
pub mod forin;
pub mod forstmt;
pub mod whl;

use crate::transform::expressions;

use super::declarations;

pub fn emit(stmt: Stmt) -> TokenStream {
    match stmt {
        Stmt::Decl(decl) => declarations::emit(decl),

        Stmt::Block(block) => block::emit(block),

        Stmt::For(for_stmt) => forstmt::emit(for_stmt),
        Stmt::ForIn(for_in_stmt) => forin::emit(for_in_stmt),
        Stmt::While(whl_stmt) => whl::emit(whl_stmt),
        Stmt::DoWhile(do_while_stmt) => dowhile::emit(do_while_stmt),

        Stmt::Empty(_) => empty::emit(),

        // We can just unwrap the node here and use the origin expression handler because it uses it in 'expr' field
        Stmt::Expr(expr) => expressions::emit(*expr.expr),

        Stmt::Debugger(_) => todo!("Debugger statement is not supported yet"),

        Stmt::With(_) => todo!("With statement is not supported yet"),

        _ => unsupported!(stmt),
    }
}
