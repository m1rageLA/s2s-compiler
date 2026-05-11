use ir::IrType;

pub(crate) fn handle(inferred: &mut Option<IrType>, saw_return: &mut bool) -> bool {
    *saw_return = true;
    super::super::unify_type(inferred, IrType::Unit)
}
