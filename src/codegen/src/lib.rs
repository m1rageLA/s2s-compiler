use ir::*;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

pub fn gen_module(m: &IrModule) -> TokenStream {
    // только переменные -> в main
    let decls = m.items.iter().filter_map(|item| match item {
        IrItem::Variable(v) => Some(gen_var_decl(v)),
        _ => None,
    });

    quote! {
        fn main() {
            #(#decls)*
        }
    }
}

fn gen_var_decl(var: &IrVariable) -> TokenStream {
    let name = Ident::new(&var.name, Span::call_site());
    let ty = gen_type(&var.ty);
    quote! {
        let #name: #ty;
    }
}

pub fn gen_type(ty: &IrType) -> TokenStream {
    match ty {
        IrType::Int => quote! { i32 },
        IrType::Str => quote! { ::std::string::String },
        IrType::Bool => quote! { bool },
        IrType::Unit => quote! { () },
        IrType::Any => quote! { /* any */ () },
    }
}
