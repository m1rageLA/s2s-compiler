use ir::{IrBinOp, IrExpression, IrType};

pub(crate) fn infer_binary(
    op: IrBinOp,
    left: &IrExpression,
    right: &IrExpression,
) -> Option<IrType> {
    match op {
        IrBinOp::Add => {
            let left_ty = super::infer_expression_type(left);
            let right_ty = super::infer_expression_type(right);

            if left_ty == Some(IrType::Str) || right_ty == Some(IrType::Str) {
                Some(IrType::Str)
            } else if left_ty == Some(IrType::Bool) || right_ty == Some(IrType::Bool) {
                None
            } else if left_ty == Some(IrType::Number)
                || right_ty == Some(IrType::Number)
                || (left_ty.is_none() && right_ty.is_none())
            {
                Some(IrType::Number)
            } else {
                None
            }
        }
        IrBinOp::Sub
        | IrBinOp::Mul
        | IrBinOp::Div
        | IrBinOp::Mod
        | IrBinOp::Exp
        | IrBinOp::LeftShift
        | IrBinOp::RightShift
        | IrBinOp::BitwiseOr
        | IrBinOp::BitwiseXor
        | IrBinOp::BitwiseAnd
        | IrBinOp::UnsignedRightShift => Some(IrType::Number),
        IrBinOp::Equal
        | IrBinOp::StrictEqual
        | IrBinOp::NotEqual
        | IrBinOp::StrictNotEqual
        | IrBinOp::LessThan
        | IrBinOp::LessThanOrEqual
        | IrBinOp::GreaterThan
        | IrBinOp::GreaterThanOrEqual
        | IrBinOp::In
        | IrBinOp::InstanceOf => Some(IrType::Bool),
        IrBinOp::LogicalOr | IrBinOp::LogicalAnd | IrBinOp::Unsupported => None,
    }
}
