use ir::{IrStmt, IrType};

use super::collect_return_types;

pub(crate) fn infer_function_return_type(body: &[IrStmt]) -> Option<IrType> {
    let mut inferred: Option<IrType> = None;
    let mut saw_return = false;

    if !collect_return_types(body, &mut inferred, &mut saw_return) {
        return None;
    }

    if saw_return {
        inferred
    } else {
        Some(IrType::Unit)
    }
}
