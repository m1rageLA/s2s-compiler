use super::*;

pub(crate) fn binary_expr_to_ir(b: &ast::BinExpr) -> IrExpression {
    IrExpression::Binary {
        op: bin_op_to_ir(&b.op),
        left: Box::new(expr_to_ir(&b.left)),
        right: Box::new(expr_to_ir(&b.right)),
    }
}

pub(crate) fn bin_op_to_ir(op: &ast::BinaryOp) -> IrBinOp {
    match op {
        ast::BinaryOp::Add => IrBinOp::Add,
        ast::BinaryOp::Sub => IrBinOp::Sub,
        ast::BinaryOp::Mul => IrBinOp::Mul,
        ast::BinaryOp::Div => IrBinOp::Div,
        ast::BinaryOp::Mod => IrBinOp::Mod,
        ast::BinaryOp::Exp => IrBinOp::Exp,
        ast::BinaryOp::EqEq => IrBinOp::Equal,
        ast::BinaryOp::EqEqEq => IrBinOp::StrictEqual,
        ast::BinaryOp::NotEq => IrBinOp::NotEqual,
        ast::BinaryOp::NotEqEq => IrBinOp::StrictNotEqual,
        ast::BinaryOp::Lt => IrBinOp::LessThan,
        ast::BinaryOp::LtEq => IrBinOp::LessThanOrEqual,
        ast::BinaryOp::Gt => IrBinOp::GreaterThan,
        ast::BinaryOp::GtEq => IrBinOp::GreaterThanOrEqual,
        ast::BinaryOp::LShift => IrBinOp::LeftShift,
        ast::BinaryOp::RShift => IrBinOp::RightShift,
        ast::BinaryOp::ZeroFillRShift => IrBinOp::UnsignedRightShift,
        ast::BinaryOp::BitOr => IrBinOp::BitwiseOr,
        ast::BinaryOp::BitXor => IrBinOp::BitwiseXor,
        ast::BinaryOp::BitAnd => IrBinOp::BitwiseAnd,
        ast::BinaryOp::LogicalOr => IrBinOp::LogicalOr,
        ast::BinaryOp::LogicalAnd => IrBinOp::LogicalAnd,
        ast::BinaryOp::In => IrBinOp::In,
        ast::BinaryOp::InstanceOf => IrBinOp::InstanceOf,
        _ => IrBinOp::Unsupported,
    }
}
