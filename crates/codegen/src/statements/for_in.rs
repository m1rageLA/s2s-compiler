use crate::function::render_type;
use crate::statements::label::label_lifetime;
use crate::{Codegen, typing};
use ir::{IrAssignOp, IrExpression, IrForInLeft, IrStmt, IrType};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::collect_stmt_tokens;

pub(crate) fn for_in_tokens(
    left: &IrForInLeft,
    right: &IrExpression,
    body: &[IrStmt],
    label: Option<&str>,
) -> TokenStream {
    typing::push_scope();

    let target_ident = format_ident!("ts_for_in_target");
    let keys_ident = format_ident!("ts_for_in_keys");
    let key_ident = format_ident!("ts_for_in_key");

    let right_tokens = right.codegen();
    let right_ty = typing::infer_expression_type(right);
    let keys_expr = keys_tokens(&target_ident, right_ty);

    let binding_tokens = render_binding(left, &key_ident);
    let body_tokens = collect_stmt_tokens(body);
    let loop_label = label.map(label_lifetime);

    typing::pop_scope();

    let loop_tokens = match loop_label {
        Some(lifetime) => quote! { #lifetime: for #key_ident in #keys_ident { #binding_tokens #(#body_tokens)* } },
        None => quote! { for #key_ident in #keys_ident { #binding_tokens #(#body_tokens)* } },
    };

    quote!({
        let #target_ident = (#right_tokens).clone();
        let #keys_ident: ::std::vec::Vec<::std::string::String> = #keys_expr;
        #loop_tokens
    })
}

fn keys_tokens(target_ident: &proc_macro2::Ident, ty: Option<IrType>) -> TokenStream {
    match ty {
        Some(IrType::Array(_)) => {
            quote! { (0 .. #target_ident.len()).map(|idx| idx.to_string()).collect::<::std::vec::Vec<_>>() }
        }
        Some(IrType::Str) => {
            quote! { (#target_ident).chars().enumerate().map(|(idx, _)| idx.to_string()).collect::<::std::vec::Vec<_>>() }
        }
        Some(IrType::Bool) | Some(IrType::Number) | Some(IrType::Unit) | Some(IrType::Object(_)) => {
            quote! { ::std::vec::Vec::<::std::string::String>::new() }
        }
        _ => quote! {
            runtime::value::ops::for_in_keys(runtime::value::into_value((#target_ident).clone()))
        },
    }
}

fn render_binding(left: &IrForInLeft, key_ident: &proc_macro2::Ident) -> TokenStream {
    match left {
        IrForInLeft::Var(var) => {
            typing::define(&var.name, var.ty);
            let ident = format_ident!("{}", var.name);
            let mutability = var.mutable.then(|| quote! { mut });
            let ty_tokens = render_type(&var.ty);
            let value_tokens =
                typing::coerce_to_type(quote! { #key_ident.clone() }, &var.ty, Some(IrType::Str));

            quote! {
                let #mutability #ident: #ty_tokens = #value_tokens;
            }
        }
        IrForInLeft::Identifier(name) => {
            let ident = format_ident!("{}", name);
            let ty = typing::lookup(name).unwrap_or(IrType::Any);
            let value_tokens =
                typing::coerce_to_type(quote! { #key_ident.clone() }, &ty, Some(IrType::Str));
            quote! {
                #ident = #value_tokens;
            }
        }
        IrForInLeft::Pattern(pattern) => {
            let binding_ident = format_ident!("ts_for_in_value");
            let binding_name = binding_ident.to_string();
            typing::define(&binding_name, IrType::Str);
            let assignment = crate::expression::assignment_tokens(
                IrAssignOp::Assign,
                pattern,
                &IrExpression::Identifier(binding_name.clone()),
            );

            quote! {
                let #binding_ident = #key_ident.clone();
                #assignment;
            }
        }
    }
}
