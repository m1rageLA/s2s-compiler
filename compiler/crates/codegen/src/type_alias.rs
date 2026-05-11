use ir::{IrTypeAlias, IrTypeAliasDef};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::function::render_type;

pub(crate) fn type_alias_tokens(alias: &IrTypeAlias) -> TokenStream {
    let name_ident = format_ident!("{}", alias.name);
    match &alias.def {
        IrTypeAliasDef::Object(fields) => {
            let copyable = fields.iter().all(|field| {
                matches!(
                    field.ty,
                    ir::IrType::Number | ir::IrType::UInt | ir::IrType::Bool | ir::IrType::Unit
                )
            });
            let field_tokens: Vec<TokenStream> = fields
                .iter()
                .map(|field| {
                    let field_ident = format_ident!("{}", field.name);
                    let field_ty = render_type(&field.ty);
                    quote! { #field_ident: #field_ty }
                })
                .collect();
            let derives = if copyable {
                quote! { #[derive(Clone, Copy, Debug, Default)] }
            } else {
                quote! { #[derive(Clone, Debug, Default)] }
            };
            quote! {
                #derives
                struct #name_ident {
                    #( #field_tokens, )*
                }
            }
        }
        IrTypeAliasDef::Alias(inner) => {
            let target = render_type(inner);
            quote! { type #name_ident = #target; }
        }
    }
}
