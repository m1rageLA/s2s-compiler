use ir::{IrForInit, IrStmt};
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
            IrStmt::While(condition, body) => {
                let condition_tokens = condition.codegen();
                let body_tokens: Vec<TokenStream> = body.iter().map(|stmt| stmt.codegen()).collect();
                quote! { while #condition_tokens { #(#body_tokens)* } }
            }
            IrStmt::DoWhile(body, condition) => {
                let condition_tokens = condition.codegen();
                let body_tokens: Vec<TokenStream> = body.iter().map(|stmt| stmt.codegen()).collect();
                quote! {
                    loop {
                        #(#body_tokens)*
                        if !(#condition_tokens) {
                            break;
                        }
                    }
                }
            }
            IrStmt::For {
                init,
                condition,
                update,
                body,
            } => {
                let init_tokens = match init {
                    Some(IrForInit::VarDecl(vars)) => {
                        let decls: Vec<TokenStream> = vars.iter().map(|var| var.codegen()).collect();
                        quote! { #(#decls)* }
                    }
                    Some(IrForInit::Expr(expr)) => {
                        let expr_tokens = expr.codegen();
                        quote! { #expr_tokens; }
                    }
                    None => TokenStream::new(),
                };

                let condition_tokens = condition.as_ref().map(|expr| expr.codegen());
                let update_tokens = if let Some(expr) = update {
                    let expr_tokens = expr.codegen();
                    quote! { #expr_tokens; }
                } else {
                    TokenStream::new()
                };
                let body_tokens: Vec<TokenStream> = body.iter().map(|stmt| stmt.codegen()).collect();

                if let Some(condition_tokens) = condition_tokens {
                    quote! {
                        {
                            #init_tokens
                            while #condition_tokens {
                                #(#body_tokens)*
                                #update_tokens
                            }
                        }
                    }
                } else {
                    quote! {
                        {
                            #init_tokens
                            loop {
                                #(#body_tokens)*
                                #update_tokens
                            }
                        }
                    }
                }
            }
            IrStmt::VarDecl(vars) => {
                let decls = vars.iter().map(|var| var.codegen());
                quote! { #(#decls)* }
            }
            IrStmt::Unsupported(reason) => unsupported_with_reason(reason),
        }
    }
}

fn unsupported_with_reason(reason: &str) -> TokenStream {
    let msg = Literal::string(&format!("unsupported statement: {reason}"));
    quote! { panic!(#msg) }
}
