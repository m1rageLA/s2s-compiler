use ir::{IrStmt, IrType};

pub(crate) fn handle(
    stmts: &[IrStmt],
    inferred: &mut Option<IrType>,
    saw_return: &mut bool,
) -> bool {
    super::collect_return_types(stmts, inferred, saw_return)
}
