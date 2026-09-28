use logger::unsupported;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use swc_ecma_ast::{ForHead, Pat};

use crate::transform::{common, expressions, statements};

/// Emits an ES5 `for … in` loop.
///
/// JavaScript enumerates property *names*, not values.  Arrays are the
/// collection representation currently supported by this code generator, so
/// enumerate their numeric property names and bind each name as a `String`.
/// The loop target is borrowed to avoid moving an identifier used after the
/// loop and is evaluated only once.
pub fn emit(for_in: swc_ecma_ast::ForInStmt) -> TokenStream {
    let target = expressions::emit(*for_in.right);
    let body = statements::emit(*for_in.body);
    let target_ident = format_ident!("__compiler_for_in_target");
    let index_ident = format_ident!("__compiler_for_in_index");
    let key_ident = format_ident!("__compiler_for_in_key");
    let binding = emit_binding(for_in.left, &key_ident);

    quote! {
        {
            let #target_ident = &(#target);
            for #index_ident in 0..#target_ident.len() {
                let #key_ident = #index_ident.to_string();
                #binding
                { #body }
            }
        }
    }
}

fn emit_binding(left: ForHead, key: &proc_macro2::Ident) -> TokenStream {
    match left {
        // A for-in declaration has a single binding in ES5.  The declaration
        // is deliberately emitted inside the loop: each iteration receives
        // the current property name, just as JavaScript assigns it each time.
        ForHead::VarDecl(var_decl) => {
            if var_decl.decls.len() != 1 {
                unsupported!(var_decl);
            }

            let declaration = var_decl.decls.into_iter().next().unwrap();
            match declaration.name {
                Pat::Ident(ident) => {
                    let ident = common::emit(ident.id);
                    quote! { let mut #ident = #key; }
                }
                _ => unsupported!(declaration.name),
            }
        }

        // An identifier/member target is an assignment in JavaScript, rather
        // than a new per-iteration binding.
        ForHead::Pat(pattern) => match *pattern {
            Pat::Ident(ident) => {
                let ident = common::emit(ident.id);
                quote! { #ident = #key; }
            }
            Pat::Expr(expr) => {
                let target = expressions::emit(*expr);
                quote! { #target = #key; }
            }
            pattern => unsupported!(pattern),
        },

        // `using` is newer than ES5 and should not survive the normalization
        // boundary.  Keep it explicit so a future normalizer regression has a
        // useful diagnostic instead of silently producing invalid Rust.
        ForHead::UsingDecl(using_decl) => unsupported!(using_decl),
    }
}
