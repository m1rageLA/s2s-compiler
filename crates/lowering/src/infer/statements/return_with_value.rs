use ir::{IrExpression, IrType};

pub(crate) fn handle(
    expr: &IrExpression,
    inferred: &mut Option<IrType>,
    saw_return: &mut bool,
) -> bool {
    *saw_return = true;
    let ty = match super::super::infer_expression_type(expr) {
        Some(value) => value,
        None => return false,
    };
    super::super::unify_type(inferred, ty)
}
