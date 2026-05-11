use ir::{IrStmt, IrType};

pub(crate) fn handle(
    then_branch: &[IrStmt],
    else_branch: Option<&[IrStmt]>,
    inferred: &mut Option<IrType>,
    saw_return: &mut bool,
) -> bool {
    if !super::collect_return_types(then_branch, inferred, saw_return) {
        return false;
    }
    if let Some(else_branch) = else_branch {
        if !super::collect_return_types(else_branch, inferred, saw_return) {
            return false;
        }
    }
    true
}
