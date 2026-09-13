pub mod variable;
pub mod function;

use proc_macro2::TokenStream;
use swc_ecma_ast::Decl;

pub fn emit(decl: Decl) -> TokenStream {
    match decl {
        Decl::Var(var_decl) => variable::emit(*var_decl),
        Decl::Fn(fn_decl) => function::emit(fn_decl),
        _ => TokenStream::new(), // Handle other declarations as needed
    }
}
