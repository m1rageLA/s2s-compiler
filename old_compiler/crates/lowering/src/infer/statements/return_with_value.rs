use ir::{IrExpression, IrType};

pub(crate) fn handle(
    expr: &IrExpression,
    inferred: &mut Option<IrType>,
    saw_return: &mut bool,
) -> bool {
    *saw_return = true;
    let inner_expr = match expr {
        IrExpression::RuntimeCall(ir::RuntimeNamespace::Value(ir::ValueCall::Coerce { expr })) => {
            expr.as_ref()
        }
        _ => expr,
    };

    let ty = match super::super::infer_expression_type(inner_expr) {
        Some(value) => value,
        None => return false,
    };
    super::super::unify_type(inferred, ty)
}
