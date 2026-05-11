use crate::{Codegen, typing};
use ir::{IrArrayKind, IrExpression, IrStmt, IrType};
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn map_tokens(target: &IrExpression, callback: &IrExpression) -> TokenStream {
    let target_tokens = target.codegen();
    let callback_tokens = callback.codegen();
    let element_kind = match typing::infer_expression_type(target) {
        Some(IrType::Array(kind)) => Some(kind),
        _ => None,
    };

    let param_ty = callback_param_type(callback);

    if let (Some(kind), Some(param)) = (element_kind, param_ty) {
        if matches!(
            kind,
            IrArrayKind::Number | IrArrayKind::Str | IrArrayKind::Bool
        ) {
            let arg = argument_tokens(&param, kind);
            let expected = infer_callback_return(callback);
            let mapped = coerce_mapped(&quote! { mapped }, expected);
            return quote! {{
                let ts_2_rs_cb = #callback_tokens;
                #target_tokens
                    .iter()
                    .map(|item| {
                        let mapped = ts_2_rs_cb(#arg);
                        #mapped
                    })
                    .collect::<::std::vec::Vec<_>>()
            }};
        }
    }

    quote! { runtime::array::map(&#target_tokens, #callback_tokens) }
}

fn callback_param_type(callback: &IrExpression) -> Option<IrType> {
    match callback {
        IrExpression::Arrow { params, .. } => params.first().map(|p| p.ty),
        IrExpression::Function(func) => func.params.first().map(|p| p.ty),
        _ => None,
    }
}

fn argument_tokens(param_ty: &IrType, element_kind: IrArrayKind) -> TokenStream {
    match param_ty {
        IrType::Any | IrType::Value => quote! { runtime::value::into_value((item).clone()) },
        IrType::Number if matches!(element_kind, IrArrayKind::Number) => quote! { (item).clone() },
        IrType::Str if matches!(element_kind, IrArrayKind::Str) => quote! { (item).clone() },
        IrType::Bool if matches!(element_kind, IrArrayKind::Bool) => quote! { (item).clone() },
        _ => quote! { runtime::value::into_value((item).clone()) },
    }
}

fn infer_callback_return(callback: &IrExpression) -> Option<IrType> {
    match callback {
        IrExpression::Arrow { params, body } => {
            typing::push_scope();
            for param in params {
                typing::define(&param.name, param.ty);
            }
            let ty = match body {
                ir::IrArrowBody::Expr(expr) => typing::infer_expression_type(expr),
                ir::IrArrowBody::Block(stmts) => infer_returns(stmts),
            };
            typing::pop_scope();
            ty
        }
        IrExpression::Function(func) => Some(func.ret),
        _ => None,
    }
}

fn infer_returns(stmts: &[IrStmt]) -> Option<IrType> {
    let mut inferred: Option<IrType> = None;
    let mut saw_return = false;

    for stmt in stmts {
        match stmt {
            IrStmt::Return(Some(expr)) => {
                let ty = typing::infer_expression_type(expr);
                if let Some(found) = ty {
                    if let Some(existing) = inferred {
                        if existing != found {
                            return None;
                        }
                    } else {
                        inferred = Some(found);
                    }
                } else {
                    return None;
                }
                saw_return = true;
            }
            IrStmt::Return(None) => {
                if let Some(existing) = inferred {
                    if existing != IrType::Unit {
                        return None;
                    }
                } else {
                    inferred = Some(IrType::Unit);
                }
                saw_return = true;
            }
            IrStmt::Block(inner) => {
                if let Some(inner_ty) = infer_returns(inner) {
                    if let Some(existing) = inferred {
                        if existing != inner_ty {
                            return None;
                        }
                    } else {
                        inferred = Some(inner_ty);
                    }
                    saw_return = true;
                }
            }
            IrStmt::If {
                then_branch,
                else_branch,
                ..
            } => {
                let then_ty = infer_returns(then_branch);
                let else_ty = else_branch.as_deref().and_then(infer_returns);
                match (then_ty, else_ty) {
                    (Some(a), Some(b)) if a == b => {
                        if let Some(existing) = inferred {
                            if existing != a {
                                return None;
                            }
                        } else {
                            inferred = Some(a);
                        }
                        saw_return = true;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    if saw_return {
        inferred
    } else {
        Some(IrType::Unit)
    }
}

fn coerce_mapped(binding: &TokenStream, expected: Option<IrType>) -> TokenStream {
    match expected {
        Some(IrType::Number) => quote! { runtime::value::into_value(#binding).into_number() },
        Some(IrType::Str) => {
            quote! { runtime::console::stringify(&runtime::value::into_value(#binding)) }
        }
        Some(IrType::Bool) => quote! { !runtime::value::ops::logical_not(#binding) },
        _ => quote! { #binding },
    }
}
