use ir::{ArrayCall, ConsoleCall, IrType, RuntimeNamespace};

pub(crate) fn infer_runtime(call: &RuntimeNamespace) -> Option<IrType> {
    match call {
        RuntimeNamespace::Console(ConsoleCall::Log(_)) => Some(IrType::Unit),
        RuntimeNamespace::Array(ArrayCall::Push { .. }) => Some(IrType::Number),
    }
}
