use ir::{
    ArrayCall, ConsoleCall, IrArrayKind, IrExpression, IrType, MathCall, RuntimeNamespace,
    StringCall, ValueCall,
};

use super::{infer_expression_type, infer_function_return_type};
use crate::context;

pub(crate) fn infer_runtime(call: &RuntimeNamespace) -> Option<IrType> {
    match call {
        RuntimeNamespace::Console(ConsoleCall::Log(_)) => Some(IrType::Unit),
        RuntimeNamespace::Array(ArrayCall::Push { .. }) => Some(IrType::Number),
        RuntimeNamespace::Array(ArrayCall::Length { target }) => match infer_expression_type(target) {
            Some(IrType::Array(IrArrayKind::Number | IrArrayKind::Str | IrArrayKind::Bool)) => {
                Some(IrType::Number)
            }
            _ => Some(IrType::Value),
        },
        RuntimeNamespace::Array(ArrayCall::Index { element, target, .. }) => match element {
            Some(IrArrayKind::Number) => Some(IrType::Number),
            _ => match infer_expression_type(target) {
                Some(IrType::Array(kind)) => match kind {
                    IrArrayKind::Number => Some(IrType::Number),
                    IrArrayKind::Str => Some(IrType::Str),
                    IrArrayKind::Bool => Some(IrType::Bool),
                    _ => Some(IrType::Value),
                },
                _ => Some(IrType::Value),
            },
        },
        RuntimeNamespace::Array(ArrayCall::Pop { target, .. }) => {
            match infer_expression_type(target) {
                Some(IrType::Array(kind)) => Some(match kind {
                    IrArrayKind::Number => IrType::Number,
                    IrArrayKind::Str => IrType::Str,
                    IrArrayKind::Bool => IrType::Bool,
                    IrArrayKind::Value => IrType::Value,
                    IrArrayKind::Any | IrArrayKind::Unknown => IrType::Any,
                }),
                _ => Some(IrType::Value),
            }
        }

        RuntimeNamespace::Array(ArrayCall::Map { target, callback }) => {
            let element_kind = infer_expression_type(target).and_then(array_kind_from_type);
            let return_ty = infer_callback_return(callback, element_kind);
            let result_kind = return_ty
                .and_then(array_kind_from_type)
                .or(element_kind.map(widen_array_kind))
                .unwrap_or(IrArrayKind::Value);
            Some(IrType::Array(result_kind))
        }
        RuntimeNamespace::Array(ArrayCall::Filter { target, callback }) => {
            // Filter preserves the element type, but we still peek at the callback so outer
            // inference can register any scoped parameter types.
            let element_kind = infer_expression_type(target).and_then(array_kind_from_type);
            let _ = infer_callback_return(callback, element_kind);
            Some(IrType::Array(element_kind.unwrap_or(IrArrayKind::Value)))
        }
        RuntimeNamespace::Array(ArrayCall::Join { .. }) => Some(IrType::Str),
        RuntimeNamespace::Value(call) => match call {
            ValueCall::Coerce { expr } => infer_expression_type(expr).or(Some(IrType::Value)),
            ValueCall::Add { left, right } => {
                let left_ty = infer_expression_type(left);
                let right_ty = infer_expression_type(right);
                if left_ty == Some(IrType::Number) && right_ty == Some(IrType::Number) {
                    Some(IrType::Number)
                } else {
                    Some(IrType::Value)
                }
            }
            ValueCall::GetProperty { .. } => Some(IrType::Value),
            ValueCall::Sub { .. }
            | ValueCall::Mul { .. }
            | ValueCall::Div { .. }
            | ValueCall::Mod { .. } => Some(IrType::Value),
            ValueCall::Equal { .. }
            | ValueCall::StrictEqual { .. }
            | ValueCall::NotEqual { .. }
            | ValueCall::StrictNotEqual { .. }
            | ValueCall::LessThan { .. }
            | ValueCall::LessThanOrEqual { .. }
            | ValueCall::GreaterThan { .. }
            | ValueCall::GreaterThanOrEqual { .. }
            | ValueCall::LogicalNot { .. } => Some(IrType::Bool),
        },
        RuntimeNamespace::String(call) => match call {
            StringCall::Length { .. } => Some(IrType::Number),
            StringCall::ToUpperCase { .. }
            | StringCall::ToLowerCase { .. }
            | StringCall::Replace { .. }
            | StringCall::Concat { .. }
            | StringCall::Slice { .. }
            | StringCall::Substr { .. } => Some(IrType::Str),
            StringCall::Split { .. } => Some(IrType::Array(IrArrayKind::Str)),
            StringCall::Includes { .. } => Some(IrType::Bool),
        },
        RuntimeNamespace::Math(call) => match call {
            MathCall::Random => Some(IrType::Number),
            MathCall::Sqrt { .. } => Some(IrType::Number),
        },
    }
}

fn array_kind_from_type(ty: IrType) -> Option<IrArrayKind> {
    match ty {
        IrType::Array(kind) => Some(kind),
        IrType::Number => Some(IrArrayKind::Number),
        IrType::Str => Some(IrArrayKind::Str),
        IrType::Bool => Some(IrArrayKind::Bool),
        IrType::Value | IrType::Any => Some(IrArrayKind::Value),
        IrType::Unit => None,
    }
}

fn widen_array_kind(kind: IrArrayKind) -> IrArrayKind {
    match kind {
        IrArrayKind::Unknown => IrArrayKind::Value,
        other => other,
    }
}

fn infer_callback_return(callback: &IrExpression, element_kind: Option<IrArrayKind>) -> Option<IrType> {
    let desired_param = element_kind.and_then(|kind| match kind {
        IrArrayKind::Number => Some(IrType::Number),
        IrArrayKind::Str => Some(IrType::Str),
        IrArrayKind::Bool => Some(IrType::Bool),
        IrArrayKind::Value | IrArrayKind::Any | IrArrayKind::Unknown => Some(IrType::Value),
    });

    match callback {
        IrExpression::Arrow { params, body } => {
            context::push_scope();
            for (idx, param) in params.iter().enumerate() {
                let ty = if idx == 0 && matches!(param.ty, IrType::Any | IrType::Value) {
                    desired_param.unwrap_or(param.ty)
                } else {
                    param.ty
                };
                context::define(&param.name, ty);
            }

            let ty = match body {
                ir::IrArrowBody::Expr(expr) => infer_expression_type(expr),
                ir::IrArrowBody::Block(stmts) => infer_function_return_type(stmts),
            };

            context::pop_scope();
            ty
        }
        IrExpression::Function(func) => {
            if let Some(param) = func.params.first() {
                let ty = if matches!(param.ty, IrType::Any | IrType::Value) {
                    desired_param.unwrap_or(param.ty)
                } else {
                    param.ty
                };
                context::push_scope();
                context::define(&param.name, ty);
                context::pop_scope();
            }
            Some(func.ret)
        }
        _ => None,
    }
}
