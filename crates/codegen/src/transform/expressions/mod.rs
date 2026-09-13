pub mod literals;
pub mod array;

use proc_macro2::TokenStream;
use swc_ecma_ast::Expr;

pub fn emit(expr: Expr) -> TokenStream {
    match expr {
        Expr::Lit(lit) => literals::emit(lit),
        Expr::Array(arr) => array::emit(arr),
        _ => todo!("Handle other expressions as needed"),
    }
}
