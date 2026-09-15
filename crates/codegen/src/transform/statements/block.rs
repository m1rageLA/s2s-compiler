use proc_macro2::TokenStream;
use quote::quote;

pub fn emit(block: swc_ecma_ast::BlockStmt) -> TokenStream {
    let stmts = block.stmts.iter().map(|stmt| super::emit(stmt.clone()));
    quote! {
        {
            #(#stmts)*
        }
    }

}
