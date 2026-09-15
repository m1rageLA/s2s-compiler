use quote::quote;

pub fn emit(ident: swc_ecma_ast::Ident) -> proc_macro2::TokenStream {
    let ident_str = proc_macro2::Ident::new(
        &ident.sym.to_string(),
        proc_macro2::Span::call_site(),
    );
    quote! {
        #ident_str
    }
}
