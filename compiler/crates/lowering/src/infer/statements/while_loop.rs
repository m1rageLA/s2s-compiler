use ir::{IrStmt, IrType};

pub(crate) fn handle(
    body: &[IrStmt],
    inferred: &mut Option<IrType>,
    saw_return: &mut bool,
) -> bool {
    super::collect_return_types(body, inferred, saw_return)
}
