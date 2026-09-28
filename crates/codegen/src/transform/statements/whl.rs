use quote::quote;

use crate::transform::{expressions, statements};

pub fn emit(whl: swc_ecma_ast::WhileStmt) -> proc_macro2::TokenStream {
    let test = expressions::emit(*whl.test);
    let body = statements::emit(*whl.body);
    quote! {
        while (#test) {
            #body
        }
    }
}