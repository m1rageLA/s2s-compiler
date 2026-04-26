use ir::{IrExpression, IrPrefixOp};

use crate::expressions::expr_to_ir;

pub fn update_expr_to_ir(u: &swc_ecma_ast::UpdateExpr) -> IrExpression {
    let left = expr_to_ir(&u.arg);
    if let ir::IrExpression::Identifier(name) = &left {
        crate::context::mark_mutated(name);
    }
    if let ir::IrExpression::Member { object, .. } = &left {
        if let ir::IrExpression::Identifier(name) = object.as_ref() {
            crate::context::mark_mutated(name);
        }
    }
    match (u.op, u.prefix) {
        (swc_ecma_ast::UpdateOp::PlusPlus, true) => ir::IrExpression::PrefixUnary {
            arg: Box::new(left),
            op: IrPrefixOp::Increment,
        },
        (swc_ecma_ast::UpdateOp::MinusMinus, true) => ir::IrExpression::PrefixUnary {
            arg: Box::new(left),
            op: IrPrefixOp::Decrement,
        },
        (swc_ecma_ast::UpdateOp::PlusPlus, false) => ir::IrExpression::PostfixUnary {
            left: Box::new(left),
            op: ir::IrPostfixOp::Increment,
        },
        (swc_ecma_ast::UpdateOp::MinusMinus, false) => ir::IrExpression::PostfixUnary {
            left: Box::new(left),
            op: ir::IrPostfixOp::Decrement,
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{assert_identifier, lower};
    use ir::{IrExpression, IrItem, IrPostfixOp, IrPrefixOp};

    #[test]
    fn handles_postfix_increment_and_decrement() {
        let ir_module = lower(
            r#"
            let counter = 0;
            counter++;
            counter--;
            ++counter;
            --counter;
        "#,
        );

        assert_eq!(ir_module.items.len(), 5);

        let increment = match &ir_module.items[1] {
            IrItem::Expression(expr) => expr,
            other => panic!("expected expression item for increment, got {other:?}"),
        };

        match increment {
            IrExpression::PostfixUnary { left, op } => {
                assert_identifier(left.as_ref(), "counter");
                assert_eq!(*op, IrPostfixOp::Increment);
            }
            other => panic!("expected postfix increment, got {other:?}"),
        }

        let decrement = match &ir_module.items[2] {
            IrItem::Expression(expr) => expr,
            other => panic!("expected expression item for decrement, got {other:?}"),
        };

        match decrement {
            IrExpression::PostfixUnary { left, op } => {
                assert_identifier(left.as_ref(), "counter");
                assert_eq!(*op, IrPostfixOp::Decrement);
            }
            other => panic!("expected postfix decrement, got {other:?}"),
        }

        let prefix_increment = match &ir_module.items[3] {
            IrItem::Expression(expr) => expr,
            other => panic!("expected expression item for prefix increment, got {other:?}"),
        };

        match prefix_increment {
            IrExpression::PrefixUnary { arg, op } => {
                assert_identifier(arg.as_ref(), "counter");
                assert_eq!(*op, IrPrefixOp::Increment);
            }
            other => panic!("expected prefix increment, got {other:?}"),
        }

        let prefix_decrement = match &ir_module.items[4] {
            IrItem::Expression(expr) => expr,
            other => panic!("expected expression item for prefix decrement, got {other:?}"),
        };

        match prefix_decrement {
            IrExpression::PrefixUnary { arg, op } => {
                assert_identifier(arg.as_ref(), "counter");
                assert_eq!(*op, IrPrefixOp::Decrement);
            }
            other => panic!("expected prefix decrement, got {other:?}"),
        }
    }
}
