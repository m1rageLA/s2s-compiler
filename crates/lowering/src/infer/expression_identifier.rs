use crate::context;
use ir::IrType;

pub(crate) fn infer_identifier(name: &str) -> Option<IrType> {
    if name == "undefined" {
        Some(IrType::Unit)
    } else {
        context::lookup(name)
    }
}
