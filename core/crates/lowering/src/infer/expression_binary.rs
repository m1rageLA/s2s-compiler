use ir::{IrBinOp, IrExpression, IrType};

pub(crate) fn infer_binary(
    op: IrBinOp,
    left: &IrExpression,
    right: &IrExpression,
) -> Option<IrType> {
    let left_ty = super::infer_expression_type(left);
    let right_ty = super::infer_expression_type(right);
    let left_numeric = matches!(left_ty, Some(IrType::Number | IrType::UInt));
    let right_numeric = matches!(right_ty, Some(IrType::Number | IrType::UInt));
    let both_uint = matches!(left_ty, Some(IrType::UInt)) && matches!(right_ty, Some(IrType::UInt));

    match op {
        IrBinOp::Add => {
            if left_ty == Some(IrType::Str) || right_ty == Some(IrType::Str) {
                Some(IrType::Str)
            } else if left_ty == Some(IrType::Bool) || right_ty == Some(IrType::Bool) {
                None
            } else if left_numeric && right_numeric {
                if both_uint {
                    Some(IrType::UInt)
                } else {
                    Some(IrType::Number)
                }
            } else if left_numeric || right_numeric || (left_ty.is_none() && right_ty.is_none()) {
                Some(IrType::Number)
            } else {
                None
            }
        }
        IrBinOp::Sub
        | IrBinOp::Div
        | IrBinOp::Exp
        | IrBinOp::LeftShift
        | IrBinOp::RightShift
        | IrBinOp::BitwiseOr
        | IrBinOp::BitwiseXor
        | IrBinOp::BitwiseAnd
        | IrBinOp::UnsignedRightShift => Some(IrType::Number),
        IrBinOp::Mul => {
            if both_uint {
                Some(IrType::UInt)
            } else {
                Some(IrType::Number)
            }
        }
        IrBinOp::Mod => {
            if left_numeric && right_numeric && both_uint {
                Some(IrType::UInt)
            } else {
                Some(IrType::Number)
            }
        }
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
