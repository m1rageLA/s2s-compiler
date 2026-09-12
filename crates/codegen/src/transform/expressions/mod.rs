pub mod literals;

use proc_macro2::TokenStream;
use swc_ecma_ast::Expr;

pub fn emit(expr: Expr) -> TokenStream {
    match expr {
        Expr::Lit(lit) => literals::emit(lit),
        _ => todo!("Handle other expressions as needed"),
    }
}
