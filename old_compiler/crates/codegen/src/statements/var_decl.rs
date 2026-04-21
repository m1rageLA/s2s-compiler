use ir::{ArrayCall, IrArrowBody, IrArrayKind, IrExpression, IrParam, IrType, IrVariable, RuntimeNamespace};
use proc_macro2::TokenStream;
use quote::quote;

use crate::{analysis, Codegen, typing};

pub fn var_decl_tokens(vars: &[IrVariable]) -> TokenStream {
    for var in vars {
        typing::define(&var.name, var.ty);
        register_function_signature(var);
        if let (IrType::Object(_), Some(IrExpression::RuntimeCall(RuntimeNamespace::Array(ArrayCall::Index { target, index, element })))) =
            (var.ty, var.value.as_ref())
        {
            if matches!(element, Some(IrArrayKind::Object(_))) {
                typing::define_object_alias(
                    &var.name,
                    typing::ObjectAlias {
                        target: (*target.as_ref()).clone(),
                        index: (*index.as_ref()).clone(),
                        element: *element,
                    },
                );
            }
        }
    }
    let decls = vars.iter().map(|var| var.codegen());
    quote! { #(#decls)* }
}

fn register_function_signature(var: &IrVariable) {
    if let Some((params, ret)) = function_signature_from_initializer(var.value.as_ref()) {
        typing::define_function_params(&var.name, &params);
        typing::define_function_return(&var.name, ret);
        if let Some(passes) = function_param_passes_from_initializer(var.value.as_ref()) {
            typing::define_function_param_passes(&var.name, &passes);
        }
    }
}

fn function_signature_from_initializer(
    expr: Option<&IrExpression>,
) -> Option<(Vec<IrParam>, IrType)> {
    match expr {
        Some(IrExpression::Function(func)) => Some((func.params.clone(), func.ret)),
        Some(IrExpression::Arrow { params, body }) => {
            let ret = infer_arrow_return(params, body).unwrap_or_else(|| match body {
                IrArrowBody::Block(_) => IrType::Unit,
                IrArrowBody::Expr(_) => IrType::Any,
            });
            Some((params.clone(), ret))
        }
        _ => None,
    }
}

fn infer_arrow_return(params: &[IrParam], body: &IrArrowBody) -> Option<IrType> {
    typing::push_scope();
    for param in params {
        typing::define(&param.name, param.ty);
    }
    let ty = typing::infer_arrow_body_type(body);
    typing::pop_scope();
    ty
}

fn function_param_passes_from_initializer(expr: Option<&IrExpression>) -> Option<Vec<typing::ParamPass>> {
    match expr {
        Some(IrExpression::Function(func)) => {
            let usages = analysis::infer_param_usages(&func.params, &func.body);
            Some(usages.iter().map(|usage| usage.pass).collect())
        }
        Some(IrExpression::Arrow { params, body }) => {
            let usages = analysis::infer_param_usages_for_arrow(params, body);
            Some(usages.iter().map(|usage| usage.pass).collect())
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ir::{IrExpression, IrLiteral, IrType, IrVariable};

    #[test]
    fn concatenates_multiple_declarations() {
        let vars = vec![
            IrVariable {
                name: "a".into(),
                mutable: false,
                ty: IrType::Number,
                value: Some(IrExpression::Literal(IrLiteral::Number(1.0))),
            },
            IrVariable {
                name: "b".into(),
                mutable: true,
                ty: IrType::Bool,
                value: Some(IrExpression::Literal(IrLiteral::Bool(false))),
            },
        ];

        let tokens = var_decl_tokens(&vars);
        let expected = quote! {
            let a: f64 = (1) as f64;
            let mut b: bool = false;
        };

        assert_eq!(tokens.to_string(), expected.to_string());
    }
}
