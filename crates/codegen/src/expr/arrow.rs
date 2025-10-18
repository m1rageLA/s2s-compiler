use ir::{IrArrowBody, IrParam};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{Codegen, function::render_type};

pub(crate) fn arrow_tokens(params: &[IrParam], body: &IrArrowBody) -> TokenStream {
    let param_bindings: Vec<TokenStream> = params
        .iter()
        .map(|param| {
            let ident = format_ident!("{}", param.name);
            let ty = render_type(&param.ty);
            quote! { #ident: #ty }
        })
        .collect();

    match body {
        IrArrowBody::Expr(expr) => {
            let params = &param_bindings;
            let expr_tokens = expr.codegen();
            quote! { move | #( #params ),* | { #expr_tokens } }
        }
        IrArrowBody::Block(stmts) => {
            let params = &param_bindings;
            let stmt_tokens = stmts.iter().map(|stmt| stmt.codegen());
            quote! { move | #( #params ),* | { #( #stmt_tokens )* } }
        }
    }
}
