use ir::IrExpression;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::Codegen;

pub(crate) fn member_tokens(object: &IrExpression, property: &str) -> TokenStream {
    let object_tokens = object.codegen();
    let property_ident = format_ident!("{}", property);
    quote! { (#object_tokens).#property_ident }
}
