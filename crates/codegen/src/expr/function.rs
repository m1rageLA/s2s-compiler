use ir::{IrFunctionExpr, IrType};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{Codegen, function::render_type};

pub(crate) fn function_expr_tokens(function: &IrFunctionExpr) -> TokenStream {
    let params: Vec<TokenStream> = function
        .params
        .iter()
        .map(|param| {
            let ident = format_ident!("{}", param.name);
            let ty = render_type(&param.ty);
            quote! { #ident: #ty }
        })
        .collect();

    let body_tokens: Vec<TokenStream> = function.body.iter().map(|stmt| stmt.codegen()).collect();

    if matches!(function.ret, IrType::Any) {
        let params = &params;
        quote! {
            move | #( #params ),* | {
                #( #body_tokens )*
            }
        }
    } else {
        let params = &params;
        let ret_ty = render_type(&function.ret);
        quote! {
            move | #( #params ),* | -> #ret_ty {
                #( #body_tokens )*
            }
        }
    }
}
