use ir::{IrExpression, IrType};

pub(crate) fn infer_conditional(
    consequent: &IrExpression,
    alternate: &IrExpression,
) -> Option<IrType> {
    let consequent_ty = super::infer_expression_type(consequent)?;
    let alternate_ty = super::infer_expression_type(alternate)?;
    if consequent_ty == alternate_ty {
        Some(consequent_ty)
    } else {
        None
    }
}
