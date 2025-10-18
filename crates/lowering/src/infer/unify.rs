use ir::IrType;

pub(crate) fn unify_type(current: &mut Option<IrType>, new_ty: IrType) -> bool {
    if let Some(existing) = current {
        *existing == new_ty
    } else {
        *current = Some(new_ty);
        true
    }
}
