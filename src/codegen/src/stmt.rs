use ir::IrStmt;
use proc_macro2::{Literal, TokenStream};
use quote::quote;

use crate::Codegen;

impl Codegen for IrStmt {
    type Output = TokenStream;

    fn codegen(&self) -> TokenStream {
        match self {
            IrStmt::Leteral(var) => var.codegen(),
            IrStmt::Expression(expr) => {
                let expr_tokens = expr.codegen();
                quote! { #expr_tokens; }
            }
            IrStmt::Return(Some(expr)) => {
                let expr_tokens = expr.codegen();
                quote! { return #expr_tokens; }
            }
            IrStmt::Return(None) => quote! { return; },
            IrStmt::Block(stmts) => {
                let stmt_tokens = stmts.iter().map(|stmt| stmt.codegen());
                quote! { { #(#stmt_tokens)* } }
            }
            IrStmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let condition_tokens = condition.codegen();
                let then_tokens = then_branch.iter().map(|stmt| stmt.codegen());
                if let Some(else_branch) = else_branch {
                    let else_tokens = else_branch.iter().map(|stmt| stmt.codegen());
                    quote! { if #condition_tokens { #(#then_tokens)* } else { #(#else_tokens)* } }
                } else {
                    quote! { if #condition_tokens { #(#then_tokens)* } }
                }
            }
            IrStmt::While(_, _) => unsupported_stmt("while statement"),
            IrStmt::VarDecl(vars) => {
                let decls = vars.iter().map(|var| var.codegen());
                quote! { #(#decls)* }
            }
            IrStmt::Unsupported(reason) => unsupported_with_reason(reason),
        }
    }
}

fn unsupported_stmt(kind: &str) -> TokenStream {
    let msg = Literal::string(&format!("codegen for {kind} not implemented"));
    quote! { panic!(#msg) }
}

fn unsupported_with_reason(reason: &str) -> TokenStream {
    let msg = Literal::string(&format!("unsupported statement: {reason}"));
    quote! { panic!(#msg) }
}
