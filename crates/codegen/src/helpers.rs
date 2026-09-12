pub fn rewerite_ident(ident: swc_ecma_ast::Ident) -> proc_macro2::Ident {
    proc_macro2::Ident::new(ident.sym.as_ref(), proc_macro2::Span::call_site())
}
