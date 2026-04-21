use ir::{IrTemplatePart, IrType};

pub(crate) fn infer_template(_parts: &[IrTemplatePart]) -> Option<IrType> {
    Some(IrType::Str)
}
