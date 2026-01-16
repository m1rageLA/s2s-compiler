use proc_macro2::TokenStream;
use quote::quote;

use super::label::label_lifetime;
use crate::statements::{
    do_while::do_while_tokens, for_in::for_in_tokens, for_loop::for_loop_tokens,
    switch_stmt::switch_tokens, while_loop::while_tokens,
};
use crate::Codegen;
use ir::IrStmt;

pub(crate) fn labeled_tokens(label: &str, body: &IrStmt) -> TokenStream {
    match body {
        IrStmt::While(condition, stmts) => while_tokens(condition, stmts, Some(label)),
        IrStmt::DoWhile(stmts, condition) => do_while_tokens(stmts, condition, Some(label)),
        IrStmt::For {
            init,
            condition,
            update,
            body,
        } => for_loop_tokens(init.as_ref(), condition.as_ref(), update.as_ref(), body, Some(label)),
        IrStmt::ForIn { left, right, body } => for_in_tokens(left, right, body, Some(label)),
        IrStmt::Switch {
            discriminant,
            cases,
        } => switch_tokens(discriminant, cases, Some(label)),
        other => {
            let inner = other.codegen();
            let lifetime = label_lifetime(label);
            quote! {
                #lifetime: loop {
                    #inner
                    break #lifetime;
                }
            }
        }
    }
}
