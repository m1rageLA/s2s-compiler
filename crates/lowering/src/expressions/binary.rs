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

#[cfg(test)]
mod tests {
    use crate::test_utils::expect_variable;
    use ir::{IrBinOp, IrExpression};

    #[test]
    fn maps_all_binary_operators() {
        let ir_module = crate::test_utils::lower(
            r#"
            const add = left + right;
            const sub = left - right;
            const mul = left * right;
            const div = left / right;
            const modulo = left % right;
            const power = left ** right;
            const eq = left == right;
            const seq = left === right;
            const neq = left != right;
            const sne = left !== right;
            const lt = left < right;
            const lte = left <= right;
            const gt = left > right;
            const gte = left >= right;
            const shl = left << right;
            const shr = left >> right;
            const ushr = left >>> right;
            const bor = left | right;
            const bxor = left ^ right;
            const band = left & right;
            const lor = left || right;
            const land = left && right;
            const inside = left in right;
            const inst = left instanceof right;
            const unsupported = left ?? right;
        "#,
        );

        let mut items = ir_module.items.iter();

        macro_rules! assert_bin_op {
            ($name:expr, $expected:expr) => {{
                let item = items.next().expect("expected another IR item");
                let variable = expect_variable(item, $name);
                let expr = variable
                    .value
                    .as_ref()
                    .expect("variable should have initializer");
                match expr {
                    IrExpression::Binary { op, .. } => assert_eq!(*op, $expected),
                    other => panic!("expected binary expression for {}, got {other:?}", $name),
                }
            }};
        }

        assert_bin_op!("add", IrBinOp::Add);
        assert_bin_op!("sub", IrBinOp::Sub);
        assert_bin_op!("mul", IrBinOp::Mul);
        assert_bin_op!("div", IrBinOp::Div);
        assert_bin_op!("modulo", IrBinOp::Mod);
        assert_bin_op!("power", IrBinOp::Exp);
        assert_bin_op!("eq", IrBinOp::Equal);
        assert_bin_op!("seq", IrBinOp::StrictEqual);
        assert_bin_op!("neq", IrBinOp::NotEqual);
        assert_bin_op!("sne", IrBinOp::StrictNotEqual);
        assert_bin_op!("lt", IrBinOp::LessThan);
        assert_bin_op!("lte", IrBinOp::LessThanOrEqual);
        assert_bin_op!("gt", IrBinOp::GreaterThan);
        assert_bin_op!("gte", IrBinOp::GreaterThanOrEqual);
        assert_bin_op!("shl", IrBinOp::LeftShift);
        assert_bin_op!("shr", IrBinOp::RightShift);
        assert_bin_op!("ushr", IrBinOp::UnsignedRightShift);
        assert_bin_op!("bor", IrBinOp::BitwiseOr);
        assert_bin_op!("bxor", IrBinOp::BitwiseXor);
        assert_bin_op!("band", IrBinOp::BitwiseAnd);
        assert_bin_op!("lor", IrBinOp::LogicalOr);
        assert_bin_op!("land", IrBinOp::LogicalAnd);
        assert_bin_op!("inside", IrBinOp::In);
        assert_bin_op!("inst", IrBinOp::InstanceOf);

        let item = items.next().expect("expected unsupported operator");
        let variable = expect_variable(item, "unsupported");
        let expr = variable
            .value
            .as_ref()
            .expect("unsupported should have initializer");
        match expr {
            IrExpression::Binary { op, .. } => assert_eq!(*op, IrBinOp::Unsupported),
            other => panic!("expected binary expression, got {other:?}"),
        }

        assert!(items.next().is_none(), "unexpected trailing items");
    }
}
