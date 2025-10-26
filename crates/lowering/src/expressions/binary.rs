use super::*;
use crate::infer;
use ir::{IrType, ValueCall};

pub(crate) fn binary_expr_to_ir(b: &ast::BinExpr) -> IrExpression {
    let left = expr_to_ir(&b.left);
    let right = expr_to_ir(&b.right);

    let left_ty = infer::infer_expression_type(&left);
    let right_ty = infer::infer_expression_type(&right);

    if requires_value_runtime(left_ty, right_ty) {
        if let Some(runtime_expr) = value_runtime_for_binary(&b.op, left.clone(), right.clone()) {
            return runtime_expr;
        }
    }

    IrExpression::Binary {
        op: bin_op_to_ir(&b.op),
        left: Box::new(left),
        right: Box::new(right),
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

fn requires_value_runtime(left: Option<IrType>, right: Option<IrType>) -> bool {
    matches!(left, Some(IrType::Any | IrType::Value | IrType::Str))
        || matches!(right, Some(IrType::Any | IrType::Value | IrType::Str))
        || left.is_none()
        || right.is_none()
}

fn value_runtime_for_binary(
    op: &ast::BinaryOp,
    left: IrExpression,
    right: IrExpression,
) -> Option<IrExpression> {
    use ast::BinaryOp;
    let call = match op {
        BinaryOp::Add => ValueCall::Add {
            left: Box::new(left),
            right: Box::new(right),
        },
        BinaryOp::Sub => ValueCall::Sub {
            left: Box::new(left),
            right: Box::new(right),
        },
        BinaryOp::Mul => ValueCall::Mul {
            left: Box::new(left),
            right: Box::new(right),
        },
        BinaryOp::Div => ValueCall::Div {
            left: Box::new(left),
            right: Box::new(right),
        },
        BinaryOp::Mod => ValueCall::Mod {
            left: Box::new(left),
            right: Box::new(right),
        },
        BinaryOp::EqEq => ValueCall::Equal {
            left: Box::new(left),
            right: Box::new(right),
        },
        BinaryOp::EqEqEq => ValueCall::StrictEqual {
            left: Box::new(left),
            right: Box::new(right),
        },
        BinaryOp::NotEq => ValueCall::NotEqual {
            left: Box::new(left),
            right: Box::new(right),
        },
        BinaryOp::NotEqEq => ValueCall::StrictNotEqual {
            left: Box::new(left),
            right: Box::new(right),
        },
        BinaryOp::Lt => ValueCall::LessThan {
            left: Box::new(left),
            right: Box::new(right),
        },
        BinaryOp::LtEq => ValueCall::LessThanOrEqual {
            left: Box::new(left),
            right: Box::new(right),
        },
        BinaryOp::Gt => ValueCall::GreaterThan {
            left: Box::new(left),
            right: Box::new(right),
        },
        BinaryOp::GtEq => ValueCall::GreaterThanOrEqual {
            left: Box::new(left),
            right: Box::new(right),
        },
        _ => return None,
    };

    Some(value_binary(call))
}

#[cfg(test)]
mod tests {
    use crate::test_utils::expect_variable;
    use ir::{IrBinOp, IrExpression, RuntimeNamespace, ValueCall};

    #[test]
    fn maps_all_binary_operators() {
        let ir_module = crate::test_utils::lower(
            r#"
            const left: number = 1;
            const right: number = 2;
            const boolLeft: boolean = true;
            const boolRight: boolean = false;
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
            const lor = boolLeft || boolRight;
            const land = boolLeft && boolRight;
            const inside = left in { value: right };
            const inst = left instanceof Number;
            const unsupported = left ?? right;
        "#,
        );

        let mut items = ir_module.items.iter();
        // Skip the helper variable declarations.
        items.next();
        items.next();
        items.next();
        items.next();

        macro_rules! assert_bin_op {
            ($name:expr, $expected:expr) => {{
                let item = items.next().expect("expected another IR item");
                let variable = expect_variable(item, $name);
                let expr = variable
                    .value
                    .as_ref()
                    .expect("variable should have initializer");
                let expr = match expr {
                    IrExpression::RuntimeCall(ir::RuntimeNamespace::Value(
                        ir::ValueCall::Coerce { expr },
                    )) => expr.as_ref(),
                    other => other,
                };
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
        {
            let item = items.next().expect("expected power item");
            let variable = expect_variable(item, "power");
            let expr = variable
                .value
                .as_ref()
                .expect("power should have initializer");
            let expr = match expr {
                IrExpression::RuntimeCall(ir::RuntimeNamespace::Value(ir::ValueCall::Coerce {
                    expr,
                })) => expr.as_ref(),
                other => other,
            };

            match expr {
                IrExpression::Call { callee, args } => {
                    match callee.as_ref() {
                        IrExpression::Member { object, property } => {
                            assert!(
                                matches!(object.as_ref(), IrExpression::Identifier(name) if name == "Math"),
                                "expected Math member access, got {object:?}"
                            );
                            assert_eq!(property, "pow");
                        }
                        other => panic!("expected Math.pow member call, got {other:?}"),
                    }
                    assert_eq!(args.len(), 2, "expected two arguments to Math.pow");
                }
                other => panic!("expected Math.pow call for power, got {other:?}"),
            }
        }
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
            IrExpression::Conditional { .. } => {
                // `??` is rewritten into a conditional expression by the downlevel pass.
            }
            other => panic!("expected binary or lowered conditional, got {other:?}"),
        }

        assert!(items.next().is_none(), "unexpected trailing items");
    }

    #[test]
    fn string_addition_uses_value_runtime() {
        let ir_module = crate::test_utils::lower(
            r#"
            const result = "foo" + "bar";
        "#,
        );

        let variable = expect_variable(&ir_module.items[0], "result");
        let value = variable
            .value
            .as_ref()
            .expect("result should have initializer");
        match value {
            IrExpression::RuntimeCall(RuntimeNamespace::Value(ValueCall::Add { left, right })) => {
                crate::test_utils::assert_string_literal(Some(left.as_ref()), "foo");
                crate::test_utils::assert_string_literal(Some(right.as_ref()), "bar");
            }
            other => panic!("expected value runtime add call, got {other:?}"),
        }
    }

    #[test]
    fn string_and_number_comparisons_use_value_runtime() {
        let ir_module = crate::test_utils::lower(
            r#"
            const eq = "5" == 5;
        "#,
        );

        let variable = expect_variable(&ir_module.items[0], "eq");
        let value = variable.value.as_ref().expect("eq should have initializer");
        match value {
            IrExpression::RuntimeCall(RuntimeNamespace::Value(ValueCall::Equal {
                left,
                right,
            })) => {
                crate::test_utils::assert_string_literal(Some(left.as_ref()), "5");
                crate::test_utils::assert_number_literal(Some(right.as_ref()), 5.0);
            }
            other => panic!("expected value runtime equality call, got {other:?}"),
        }
    }

    #[test]
    fn routes_any_operands_through_value_runtime() {
        let ir_module = crate::test_utils::lower(
            r#"
            function demo(value: any) {
                const result = value + 1;
            }
        "#,
        );

        let function = match &ir_module.items[0] {
            ir::IrItem::Function(func) => func,
            other => panic!("expected function item, got {other:?}"),
        };

        let decl = match &function.body[0] {
            ir::IrStmt::VarDecl(vars) => vars,
            other => panic!("expected variable declaration, got {other:?}"),
        };

        let result = &decl[0];
        assert_eq!(result.name, "result");
        let expr = result
            .value
            .as_ref()
            .expect("result should have initializer");

        match expr {
            IrExpression::RuntimeCall(RuntimeNamespace::Value(ValueCall::Add { .. })) => {}
            other => panic!("expected value runtime call for dynamic addition, got {other:?}"),
        }
    }
}
