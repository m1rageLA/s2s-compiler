use ir::{ArrayCall, ConsoleCall, IrArrayKind, IrType, RuntimeNamespace};

pub(crate) fn infer_runtime(call: &RuntimeNamespace) -> Option<IrType> {
    match call {
        RuntimeNamespace::Console(ConsoleCall::Log(_)) => Some(IrType::Unit),
        RuntimeNamespace::Array(ArrayCall::Push { .. }) => Some(IrType::Number),
        RuntimeNamespace::Array(ArrayCall::Length { .. }) => Some(IrType::Number),
        RuntimeNamespace::Array(ArrayCall::Index { element, .. }) => match element {
            Some(IrArrayKind::Number) => Some(IrType::Number),
            _ => Some(IrType::Any),
        },
    }
}
