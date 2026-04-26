use ir::{IrFunctionExpr, IrType};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{Codegen, analysis, function::render_type, typing};

pub(crate) fn function_expr_tokens(function: &IrFunctionExpr) -> TokenStream {
    typing::push_scope();
    let param_usages = analysis::infer_param_usages(&function.params, &function.body);
    for (param, usage) in function.params.iter().zip(param_usages.iter()) {
        typing::define(&param.name, param.ty);
        typing::define_binding_pass(&param.name, usage.pass);
    }
    typing::push_return_type(function.ret);

    let params: Vec<TokenStream> = function
        .params
        .iter()
        .zip(param_usages.iter())
        .map(|(param, usage)| {
            let ident = format_ident!("{}", param.name);
            let ty = render_param_type(&param.ty, usage.pass);
            let mutability = (usage.mutated && matches!(usage.pass, typing::ParamPass::Value))
                .then(|| quote! { mut });
            quote! { #mutability #ident: #ty }
        })
        .collect();

    let body_tokens: Vec<TokenStream> = function.body.iter().map(|stmt| stmt.codegen()).collect();

    typing::pop_return_type();
    typing::pop_scope();

    if matches!(function.ret, IrType::Any) {
        let params = &params;
        quote! {
            | #( #params ),* | {
                #( #body_tokens )*
            }
        }
    } else {
        let params = &params;
        let ret_ty = render_type(&function.ret);
        quote! {
            | #( #params ),* | -> #ret_ty {
                #( #body_tokens )*
            }
        }
    }
}

fn render_param_type(ty: &IrType, pass: typing::ParamPass) -> TokenStream {
    if let IrType::Array(_) = ty {
        let inner = render_type(ty);
        match pass {
            typing::ParamPass::MutRef => quote! { &mut #inner },
            typing::ParamPass::Ref => quote! { & #inner },
            typing::ParamPass::Value => inner,
        }
    } else {
        render_type(ty)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::function::render_type;
    use ir::{IrExpression, IrFunctionExpr, IrLiteral, IrParam, IrStmt, IrType};
    use quote::ToTokens;
    use syn::{Expr, ExprClosure, Lit, Pat, PatType, ReturnType, Stmt};

    fn parse_closure(tokens: TokenStream) -> ExprClosure {
        match syn::parse2::<Expr>(tokens).expect("closure expression should parse") {
            Expr::Closure(closure) => closure,
            _ => panic!("expected closure expression"),
        }
    }

    #[test]
    fn function_expression_with_return_type_preserves_signature() {
        let func = IrFunctionExpr {
            name: None,
            params: vec![IrParam {
                name: "input".into(),
                ty: IrType::Str,
            }],
            ret: IrType::Bool,
            body: vec![IrStmt::Return(Some(IrExpression::Literal(
                IrLiteral::Bool(true),
            )))],
        };

        let closure = parse_closure(function_expr_tokens(&func));
        assert!(
            closure.capture.is_none(),
            "closure should not force move capture"
        );
        assert_eq!(closure.inputs.len(), 1);

        let typed_param = match &closure.inputs[0] {
            Pat::Type(PatType { ty, .. }) => ty.as_ref(),
            _ => panic!("parameter should be typed"),
        };
        let param_ty = typed_param.to_token_stream().to_string();
        assert_eq!(param_ty, render_type(&IrType::Str).to_string());

        match &closure.output {
            ReturnType::Type(_, ty) => {
                assert_eq!(ty.to_token_stream().to_string(), "bool");
            }
            ReturnType::Default => panic!("expected explicit return type"),
        }

        let stmts = match closure.body.as_ref() {
            Expr::Block(block) => &block.block.stmts,
            _ => panic!("expected block body"),
        };

        match stmts.first() {
            Some(Stmt::Expr(expr, _)) => match expr {
                Expr::Return(ret) => {
                    let inner = ret.expr.as_ref().expect("return expression");
                    match inner.as_ref() {
                        Expr::Lit(lit) => match &lit.lit {
                            Lit::Bool(value) => assert!(value.value()),
                            _ => panic!("expected boolean literal return"),
                        },
                        _ => panic!("expected literal in return"),
                    }
                }
                _ => panic!("expected return statement in closure body"),
            },
            _ => panic!("expected closure body"),
        }
    }

    #[test]
    fn function_expression_without_return_type_for_any() {
        let func = IrFunctionExpr {
            name: None,
            params: vec![IrParam {
                name: "value".into(),
                ty: IrType::Number,
            }],
            ret: IrType::Any,
            body: vec![IrStmt::Return(Some(IrExpression::Identifier(
                "value".into(),
            )))],
        };

        let closure = parse_closure(function_expr_tokens(&func));
        assert!(matches!(closure.output, ReturnType::Default));
    }
}
