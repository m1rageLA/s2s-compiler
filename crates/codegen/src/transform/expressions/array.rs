use proc_macro2::TokenStream;
use quote::quote;

pub fn emit(array_expr: swc_ecma_ast::ArrayLit) -> TokenStream {
    let elems = array_expr.elems.into_iter().map(|elem| {
        match elem {
            Some(expr) => super::emit(*expr.expr),
            None => quote! { /* empty slot */ },
        }
    });
    quote! {
        [#(#elems),*]
    }
}
