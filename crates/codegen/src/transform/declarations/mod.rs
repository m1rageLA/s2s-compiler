pub mod variable;

use proc_macro2::TokenStream;
use swc_ecma_ast::Decl;

pub fn emit(decl: Decl) -> TokenStream {
    match decl {
        Decl::Var(var_decl) => variable::emit(*var_decl),
        _ => TokenStream::new(), // Handle other declarations as needed
    }
}
