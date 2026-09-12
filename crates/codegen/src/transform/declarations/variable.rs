use proc_macro2::TokenStream;
use quote::quote;
use swc_ecma_ast::{Pat, VarDecl};

use crate::{helpers, transform::expressions};

pub fn emit(var_decl: VarDecl) -> TokenStream {
    let declarations = var_decl.decls.iter().map(|decl| {
        let id = match &decl.name {
            Pat::Ident(ident) => helpers::rewerite_ident(ident.id.clone()),
            _ => panic!("Unsupported pattern in variable declaration"),
        };

        let init_expr = match &decl.init {
            // let x = 5; // example with initializer
            Some(expr) => expressions::emit(*expr.clone()),
            // let x; // example without initializer
            None => quote! { () },
        };

        quote! {
            let mut #id = #init_expr;
        }
    });

    quote! {
        #(#declarations)*
    }
}
