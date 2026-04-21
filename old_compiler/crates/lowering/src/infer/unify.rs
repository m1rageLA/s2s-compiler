use ir::IrType;

pub(crate) fn unify_type(current: &mut Option<IrType>, new_ty: IrType) -> bool {
    if let Some(existing) = current {
        if *existing == new_ty {
            true
        } else if matches!(
            (*existing, new_ty),
            (IrType::Number, IrType::UInt) | (IrType::UInt, IrType::Number)
        ) {
            *existing = IrType::Number;
            true
        } else {
            false
        }
    } else {
        *current = Some(new_ty);
        true
    }
}
