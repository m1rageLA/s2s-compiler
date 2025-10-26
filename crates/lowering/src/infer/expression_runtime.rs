use ir::{ArrayCall, ConsoleCall, IrArrayKind, IrType, RuntimeNamespace, ValueCall};

use super::infer_expression_type;

pub(crate) fn infer_runtime(call: &RuntimeNamespace) -> Option<IrType> {
    match call {
        RuntimeNamespace::Console(ConsoleCall::Log(_)) => Some(IrType::Unit),
        RuntimeNamespace::Array(ArrayCall::Push { .. }) => Some(IrType::Number),
        RuntimeNamespace::Array(ArrayCall::Length { .. }) => Some(IrType::Number),
        RuntimeNamespace::Array(ArrayCall::Index { element, .. }) => match element {
            Some(IrArrayKind::Number) => Some(IrType::Number),
            _ => Some(IrType::Any),
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

        RuntimeNamespace::Array(ArrayCall::Map { target, .. }) => {
            match infer_expression_type(target) {
                Some(IrType::Array(kind)) => Some(IrType::Array(kind)),
                _ => Some(IrType::Array(IrArrayKind::Value)),
            }
        }
        RuntimeNamespace::Array(ArrayCall::Filter { target, .. }) => {
            match infer_expression_type(target) {
                Some(IrType::Array(kind)) => Some(IrType::Array(kind)),
                _ => Some(IrType::Array(IrArrayKind::Value)),
            }
        }
        RuntimeNamespace::Value(call) => match call {
            ValueCall::Coerce { .. } => Some(IrType::Value),
            ValueCall::Add { .. } => Some(IrType::Value),
            ValueCall::Sub { .. }
            | ValueCall::Mul { .. }
            | ValueCall::Div { .. }
            | ValueCall::Mod { .. } => Some(IrType::Number),
            ValueCall::Equal { .. }
            | ValueCall::StrictEqual { .. }
            | ValueCall::NotEqual { .. }
            | ValueCall::StrictNotEqual { .. }
            | ValueCall::LessThan { .. }
            | ValueCall::LessThanOrEqual { .. }
            | ValueCall::GreaterThan { .. }
            | ValueCall::GreaterThanOrEqual { .. } => Some(IrType::Bool),
        },
    }
}
