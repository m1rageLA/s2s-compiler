use ir::{IrFunction, IrType, IrVariable};
use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};

use crate::Codegen;

impl Codegen for IrFunction {
    type Output = TokenStream;

    fn codegen(&self) -> TokenStream {
        let name = format_ident!("{}", self.name);
        let params = self.params.iter().map(|param| {
            let ident = format_ident!("{}", param.name);
            let ty = render_type(&param.ty);
            quote! { #ident: #ty }
        });
        let return_ty = render_type(&self.ret);
        let body = self.body.iter().map(|stmt| stmt.codegen());

        quote! {
            fn #name( #( #params ),* ) -> #return_ty {
                #( #body )*
            }
        }
    }
}

impl Codegen for IrVariable {
    type Output = TokenStream;

    fn codegen(&self) -> TokenStream {
        let ident = format_ident!("{}", self.name);
        let ty = render_type(&self.ty);
        let value = self
            .value
            .as_ref()
            .map(|expr| expr.codegen())
            .unwrap_or_else(|| default_value(&self.ty));

        quote! {
            let #ident: #ty = #value;
        }
    }
}

fn render_type(ty: &IrType) -> TokenStream {
    match ty {
        IrType::Int => quote! { i128 },
        IrType::Str => quote! { ::std::string::String },
        IrType::Bool => quote! { bool },
        IrType::Unit => quote! { () },
        IrType::Any => quote! { ::std::boxed::Box<dyn ::std::any::Any> },
    }
}

fn default_value(ty: &IrType) -> TokenStream {
    match ty {
        IrType::Int => quote! { 0i128 },
        IrType::Str => quote! { ::std::string::String::new() },
        IrType::Bool => quote! { false },
        IrType::Unit => quote! { () },
        IrType::Any => {
            let msg = Literal::string("uninitialized value for `any` type");
            quote! { panic!(#msg) }
        }
    }
}
