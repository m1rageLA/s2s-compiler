use quote::quote;

use crate::transform::expressions;

pub fn emit(for_stmt: swc_ecma_ast::ForStmt) -> proc_macro2::TokenStream {
    let cond = match for_stmt.test {
        Some(test_expr) => expressions::emit(*test_expr),
        None => quote! { true },
    };
    let body = super::emit(*for_stmt.body);
    let update = match for_stmt.update {
        Some(update_expr) => expressions::emit(*update_expr),
        None => quote! { },
    };
    
    quote! {
        loop {
            if #cond {
                #body
            } else {
                break;
            }

            #update
        }
    }
}
