pub mod literals;
pub mod array;
pub mod identifier;

use proc_macro2::TokenStream;
use logger::unsupported;
use swc_ecma_ast::Expr;

pub fn emit(expr: Expr) -> TokenStream {
    match expr {
        Expr::Lit(lit) => literals::emit(lit),
        Expr::Array(arr) => array::emit(arr),
        Expr::Ident(ident) => identifier::emit(ident),
        _ => unsupported!(expr),
    }
}
