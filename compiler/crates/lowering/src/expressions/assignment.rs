use super::*;
use crate::context;
use crate::expressions::coerce_to_value;
use ir::{IrType, RuntimeNamespace};

pub(crate) fn assignment_expr_to_ir(assign: &ast::AssignExpr) -> IrExpression {
    let left = assignment_target_to_ir(&assign.left);

    if let IrExpression::Identifier(name) = &left {
        context::mark_mutated(name);
    }
    if let IrExpression::Member { object, .. } = &left {
        if let IrExpression::Identifier(name) = object.as_ref() {
            context::mark_mutated(name);
        }
    }
    let mut right = expr_to_ir(&assign.right);

    // If assigning to a simple identifier which is declared as dynamic (`Any`/`Value`),
    // coerce the RHS to Value — but skip coercion for simple literal/template RHS
    // expressions so tests and literals stay as-is.
    if let IrExpression::Identifier(name) = &left {
        if let Some(ty) = context::lookup(name) {
            if matches!(ty, IrType::Value | IrType::Any) {
                match &right {
                    IrExpression::Literal(_)
                    | IrExpression::Template(_)
                    | IrExpression::RuntimeCall(RuntimeNamespace::Value(_)) => {}
                    _ => {
                        right = coerce_to_value(right);
                    }
                }
            }
        }
    }

    IrExpression::Assignment {
        op: assign_op_to_ir(assign.op),
        left: Box::new(left),
        right: Box::new(right),
    }
}

fn assignment_target_to_ir(target: &ast::AssignTarget) -> IrExpression {
    match target {
        ast::AssignTarget::Simple(simple) => simple_assign_target_to_ir(simple),
        ast::AssignTarget::Pat(_) => {
            IrExpression::Identifier("unsupported_assignment_target".into())
        }
    }
}

fn simple_assign_target_to_ir(target: &ast::SimpleAssignTarget) -> IrExpression {
    use ast::SimpleAssignTarget::*;
    match target {
        Ident(binding_ident) => IrExpression::Identifier(binding_ident.id.sym.to_string()),
        Member(member) => lower_member_expr(member),
        Paren(paren) => expr_to_ir(&paren.expr),
        TsAs(ts_as) => expr_to_ir(&ts_as.expr),
        TsSatisfies(ts_satisfies) => expr_to_ir(&ts_satisfies.expr),
        TsNonNull(ts_non_null) => expr_to_ir(&ts_non_null.expr),
        TsTypeAssertion(ts_type_assertion) => expr_to_ir(&ts_type_assertion.expr),
        TsInstantiation(ts_instantiation) => expr_to_ir(&ts_instantiation.expr),
        _ => IrExpression::Identifier("unsupported_assignment_target".into()),
    }
}

fn assign_op_to_ir(op: ast::AssignOp) -> IrAssignOp {
    match op {
        ast::AssignOp::Assign => IrAssignOp::Assign,
        ast::AssignOp::AddAssign => IrAssignOp::AddAssign,
        ast::AssignOp::SubAssign => IrAssignOp::SubAssign,
        ast::AssignOp::MulAssign => IrAssignOp::MulAssign,
        ast::AssignOp::DivAssign => IrAssignOp::DivAssign,
        ast::AssignOp::ModAssign => IrAssignOp::ModAssign,
        ast::AssignOp::ExpAssign => IrAssignOp::ExpAssign,
        ast::AssignOp::LShiftAssign => IrAssignOp::LeftShiftAssign,
        ast::AssignOp::RShiftAssign => IrAssignOp::RightShiftAssign,
        ast::AssignOp::ZeroFillRShiftAssign => IrAssignOp::UnsignedRightShiftAssign,
        ast::AssignOp::BitOrAssign => IrAssignOp::BitwiseOrAssign,
        ast::AssignOp::BitXorAssign => IrAssignOp::BitwiseXorAssign,
        ast::AssignOp::BitAndAssign => IrAssignOp::BitwiseAndAssign,
        ast::AssignOp::AndAssign => IrAssignOp::LogicalAndAssign,
        ast::AssignOp::OrAssign => IrAssignOp::LogicalOrAssign,
        ast::AssignOp::NullishAssign => IrAssignOp::NullishCoalesceAssign,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{assert_identifier, assert_number_literal, expect_variable, lower};
    use ir::{IrExpression, IrItem, IrLiteral};

    fn expect_assignment(expr: &IrExpression) -> (&IrExpression, &IrExpression, IrAssignOp) {
        match expr {
            IrExpression::Assignment { op, left, right } => (left, right, *op),
            other => panic!("expected assignment expression, got {other:?}"),
        }
    }

    #[test]
    fn lowers_simple_assignment() {
        let ir_module = lower(
            r#"
            let value = 0;
            value = 5;
        "#,
        );

        match ir_module.items.last() {
            Some(IrItem::Expression(expr)) => {
                let (left, right, op) = expect_assignment(expr);
                assert_identifier(left, "value");
                assert_eq!(op, IrAssignOp::Assign);
                match right {
                    IrExpression::Literal(IrLiteral::Number(num)) => {
                        assert!((*num - 5.0).abs() < f64::EPSILON);
                    }
                    other => panic!("expected numeric literal, got {other:?}"),
                }
            }
            other => panic!("expected expression item, got {other:?}"),
        }
    }

    #[test]
    fn lowers_compound_assignment() {
        let ir_module = lower(
            r#"
            let counter = 1;
            counter += 2;
        "#,
        );

        match ir_module.items.last() {
            Some(IrItem::Expression(expr)) => {
                let (left, right, op) = expect_assignment(expr);
                assert_identifier(left, "counter");
                assert_eq!(op, IrAssignOp::AddAssign);
                assert_number_literal(Some(right), 2.0);
            }
            other => panic!("expected expression item, got {other:?}"),
        }
    }

    #[test]
    fn respects_parenthesized_lhs() {
        let ir_module = lower(
            r#"
            let counter = 0;
            (counter) *= 3;
        "#,
        );

        match ir_module.items.last() {
            Some(IrItem::Expression(expr)) => {
                let (left, right, op) = expect_assignment(expr);
                assert_identifier(left, "counter");
                assert_eq!(op, IrAssignOp::MulAssign);
                assert_number_literal(Some(right), 3.0);
            }
            other => panic!("expected expression item, got {other:?}"),
        }
    }

    #[test]
    fn lowers_assignment_expression_in_initializer() {
        let ir_module = lower(
            r#"
            let counter = 1;
            const result = (counter += 2);
        "#,
        );

        assert_eq!(ir_module.items.len(), 2);
        let result_var = expect_variable(&ir_module.items[1], "result");
        let value = result_var
            .value
            .as_ref()
            .expect("result should have initializer");

        let value_expr = match value {
            IrExpression::RuntimeCall(ir::RuntimeNamespace::Value(ir::ValueCall::Coerce {
                expr,
            })) => expr.as_ref(),
            other => other,
        };

        let (left, right, op) = expect_assignment(value_expr);
        assert_identifier(left, "counter");
        assert_eq!(op, IrAssignOp::AddAssign);
        assert_number_literal(Some(right), 2.0);
    }

    #[test]
    fn lowers_member_assignment_targets() {
        let ir_module = lower(
            r#"
            let holder = get_holder();
            holder.value -= 4;
        "#,
        );

        match ir_module.items.last() {
            Some(IrItem::Expression(expr)) => {
                let (left, right, op) = expect_assignment(expr);
                match left {
                    IrExpression::Member { object, property } => {
                        assert_identifier(object, "holder");
                        assert_eq!(property, "value");
                    }
                    other => panic!("expected member expression, got {other:?}"),
                }
                assert_eq!(op, IrAssignOp::SubAssign);
                assert_number_literal(Some(right), 4.0);
            }
            other => panic!("expected expression item, got {other:?}"),
        }
    }
}
