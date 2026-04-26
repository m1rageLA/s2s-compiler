use ir::IrExpression;
use proc_macro2::TokenStream;
use quote::quote;

use crate::{Codegen, analysis, expression::object_struct_literal_tokens, typing};

pub(crate) fn call_tokens(callee: &IrExpression, args: &[IrExpression]) -> TokenStream {
    let callee_tokens = callee.codegen();
    let callee_ty = typing::infer_expression_type(callee);
    let arg_tokens: Vec<TokenStream> = if let IrExpression::Identifier(name) = callee {
        if let Some(param_types) = typing::lookup_function_params(name) {
            let param_passes = typing::lookup_function_param_passes(name);
            args.iter()
                .enumerate()
                .map(|(idx, arg)| {
                    let tokens = if let Some(target_ty) = param_types.get(idx) {
                        match (target_ty, arg) {
                            (ir::IrType::Object(id), IrExpression::Object(props)) => {
                                object_struct_literal_tokens(*id, props)
                            }
                            _ => arg.codegen(),
                        }
                    } else {
                        arg.codegen()
                    };
                    let base = match param_passes
                        .as_ref()
                        .and_then(|passes| passes.get(idx).copied())
                    {
                        Some(typing::ParamPass::MutRef) => {
                            if let IrExpression::Identifier(name) = arg {
                                if matches!(
                                    typing::lookup_binding_pass(name),
                                    Some(typing::ParamPass::MutRef)
                                ) {
                                    quote! { (#tokens) }
                                } else {
                                    quote! { &mut (#tokens) }
                                }
                            } else {
                                quote! { &mut (#tokens) }
                            }
                        }
                        Some(typing::ParamPass::Ref) => quote! { & (#tokens) },
                        _ => {
                            if typing::expr_is_copy_type(arg) {
                                quote! { (#tokens) }
                            } else {
                                quote! { (#tokens).clone() }
                            }
                        }
                    };

                    if let Some(target_ty) = param_types.get(idx) {
                        let expr_ty = typing::infer_expression_type(arg);
                        if matches!(
                            param_passes
                                .as_ref()
                                .and_then(|passes| passes.get(idx).copied()),
                            Some(typing::ParamPass::Ref | typing::ParamPass::MutRef)
                        ) {
                            base
                        } else {
                            typing::coerce_to_type(base, target_ty, expr_ty)
                        }
                    } else {
                        base
                    }
                })
                .collect()
        } else {
            default_args(args, callee_ty)
        }
    } else if let IrExpression::Arrow { params, body } = callee {
        let usages = analysis::infer_param_usages_for_arrow(params, body);
        call_args_with_passes(args, params, &usages)
    } else if let IrExpression::Function(func) = callee {
        let usages = analysis::infer_param_usages(&func.params, &func.body);
        call_args_with_passes(args, &func.params, &usages)
    } else {
        default_args(args, callee_ty)
    };

    quote! { (#callee_tokens)( #( #arg_tokens ),* ) }
}

fn call_args_with_passes(
    args: &[IrExpression],
    params: &[ir::IrParam],
    usages: &[analysis::ParamUsage],
) -> Vec<TokenStream> {
    args.iter()
        .enumerate()
        .map(|(idx, arg)| {
            let tokens = match params.get(idx).map(|param| param.ty) {
                Some(ir::IrType::Object(id)) => match arg {
                    IrExpression::Object(props) => object_struct_literal_tokens(id, props),
                    _ => arg.codegen(),
                },
                _ => arg.codegen(),
            };
            let pass = usages
                .get(idx)
                .map(|usage| usage.pass)
                .unwrap_or(typing::ParamPass::Value);
            let base = match pass {
                typing::ParamPass::MutRef => {
                    if let IrExpression::Identifier(name) = arg {
                        if matches!(
                            typing::lookup_binding_pass(name),
                            Some(typing::ParamPass::MutRef)
                        ) {
                            quote! { (#tokens) }
                        } else {
                            quote! { &mut (#tokens) }
                        }
                    } else {
                        quote! { &mut (#tokens) }
                    }
                }
                typing::ParamPass::Ref => quote! { & (#tokens) },
                typing::ParamPass::Value => {
                    if typing::expr_is_copy_type(arg) {
                        quote! { (#tokens) }
                    } else {
                        quote! { (#tokens).clone() }
                    }
                }
            };

            if let Some(target_ty) = params.get(idx).map(|param| param.ty) {
                let expr_ty = typing::infer_expression_type(arg);
                if matches!(pass, typing::ParamPass::Ref | typing::ParamPass::MutRef) {
                    base
                } else {
                    typing::coerce_to_type(base, &target_ty, expr_ty)
                }
            } else {
                base
            }
        })
        .collect()
}

fn default_args(args: &[IrExpression], callee_ty: Option<ir::IrType>) -> Vec<TokenStream> {
    let coerce_value = matches!(callee_ty, Some(ir::IrType::Any | ir::IrType::Value));
    args.iter()
        .map(|arg| {
            let tokens = arg.codegen();
            let base = if typing::expr_is_copy_type(arg) {
                quote! { (#tokens) }
            } else {
                quote! { (#tokens).clone() }
            };
            if coerce_value {
                typing::coerce_to_type(base, &ir::IrType::Value, typing::infer_expression_type(arg))
            } else {
                base
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ir::{IrExpression, IrLiteral};
    use syn::{Expr, ExprCall};

    fn parse_call(tokens: TokenStream) -> ExprCall {
        match syn::parse2::<Expr>(tokens).expect("call expression should parse") {
            Expr::Call(call) => call,
            _ => panic!("expected call expression"),
        }
    }

    #[test]
    fn call_tokens_wraps_callee_and_arguments() {
        let callee = IrExpression::Identifier("make_value".into());
        let args = vec![
            IrExpression::Literal(IrLiteral::Number(1.0)),
            IrExpression::Literal(IrLiteral::Bool(true)),
        ];

        let call = parse_call(call_tokens(&callee, &args));

        let callee_ident = match call.func.as_ref() {
            Expr::Paren(paren) => match paren.expr.as_ref() {
                Expr::Path(path) => path.path.get_ident().map(|ident| ident.to_string()),
                _ => None,
            },
            _ => None,
        }
        .expect("callee should be identifier");
        assert_eq!(callee_ident, "make_value");
        assert_eq!(call.args.len(), 2);
    }
}
