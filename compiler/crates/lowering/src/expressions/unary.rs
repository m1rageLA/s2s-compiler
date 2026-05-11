use super::*;
use crate::context;
use ir::{IrDeleteProperty, IrDeleteTarget, IrExpression, IrUnaryOp, RuntimeNamespace, ValueCall};

pub fn unary_expr_to_ir(u: &ast::UnaryExpr) -> IrExpression {
    match u.op {
        ast::UnaryOp::Minus => match expr_to_ir(&u.arg) {
            IrExpression::Literal(IrLiteral::Number(value)) => {
                IrExpression::Literal(IrLiteral::Number(-value))
            }
            _ => IrExpression::Binary {
                op: IrBinOp::Sub,
                left: Box::new(IrExpression::Literal(IrLiteral::Number(0.0))),
                right: Box::new(expr_to_ir(&u.arg)),
            },
        },
        ast::UnaryOp::Plus => expr_to_ir(&u.arg),
        ast::UnaryOp::Bang => {
            IrExpression::RuntimeCall(RuntimeNamespace::Value(ValueCall::LogicalNot {
                expr: Box::new(expr_to_ir(&u.arg)),
            }))
        }
        ast::UnaryOp::Tilde => IrExpression::Unary {
            op: IrUnaryOp::BitwiseNot,
            expr: Box::new(expr_to_ir(&u.arg)),
        },
        ast::UnaryOp::TypeOf => IrExpression::Unary {
            op: IrUnaryOp::TypeOf,
            expr: Box::new(expr_to_ir(&u.arg)),
        },
        ast::UnaryOp::Void => IrExpression::Unary {
            op: IrUnaryOp::Void,
            expr: Box::new(expr_to_ir(&u.arg)),
        },
        ast::UnaryOp::Delete => delete_expr_to_ir(&u.arg),
    }
}

fn delete_expr_to_ir(expr: &ast::Expr) -> IrExpression {
    match expr {
        ast::Expr::Member(member) => {
            if let ast::Expr::Ident(ident) = member.obj.as_ref() {
                context::mark_mutated(&ident.sym.to_string());
            }
            let object = expr_to_ir(&member.obj);

            let property = match &member.prop {
                ast::MemberProp::Ident(ident) => IrDeleteProperty::Static(ident.sym.to_string()),
                ast::MemberProp::PrivateName(name) => {
                    IrDeleteProperty::Static(format!("#{}", name.name))
                }
                ast::MemberProp::Computed(comp) => {
                    IrDeleteProperty::Dynamic(Box::new(expr_to_ir(&comp.expr)))
                }
            };

            IrExpression::Delete(IrDeleteTarget::Property {
                object: Box::new(object),
                property,
            })
        }
        other => IrExpression::Delete(IrDeleteTarget::Expr(Box::new(expr_to_ir(other)))),
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{assert_identifier, expect_variable, lower};
    use ir::{
        IrBinOp, IrDeleteProperty, IrDeleteTarget, IrExpression, IrLiteral, RuntimeNamespace,
        ValueCall,
    };

    fn unwrap_value(expr: &IrExpression) -> &IrExpression {
        match expr {
            IrExpression::RuntimeCall(ir::RuntimeNamespace::Value(ir::ValueCall::Coerce {
                expr,
            })) => expr.as_ref(),
            other => other,
        }
    }

    #[test]
    fn lowers_unary_minus_and_plus() {
        let ir_module = lower(
            r#"
            const literal = -1;
            const computed = -value;
            const positive = +value;
            const negated = !value;
            const bitwise = ~value;
            const typeofed = typeof value;
            const voided = void value;
            const record = { name: "ok" };
            const removed = delete record.name;
        "#,
        );

        assert_eq!(ir_module.items.len(), 9);

        let literal = expect_variable(&ir_module.items[0], "literal");
        match literal
            .value
            .as_ref()
            .expect("literal should have initializer")
        {
            IrExpression::Literal(IrLiteral::Number(value)) => assert_eq!(*value, -1.0),
            other => panic!("expected folded numeric literal, got {other:?}"),
        }

        let computed = expect_variable(&ir_module.items[1], "computed");
        match computed
            .value
            .as_ref()
            .expect("computed should have initializer")
        {
            IrExpression::Binary { op, left, right } => {
                assert_eq!(*op, IrBinOp::Sub);
                match left.as_ref() {
                    IrExpression::Literal(IrLiteral::Number(value)) => assert_eq!(*value, 0.0),
                    other => panic!("expected zero literal on left side, got {other:?}"),
                }
                assert_identifier(right, "value");
            }
            other => panic!("expected binary subtraction expansion, got {other:?}"),
        }

        let positive = expect_variable(&ir_module.items[2], "positive");
        let positive_value = unwrap_value(
            positive
                .value
                .as_ref()
                .expect("positive should have initializer"),
        );
        match positive_value {
            IrExpression::Identifier(name) => assert_eq!(name, "value"),
            other => panic!("expected identity for unary plus, got {other:?}"),
        }

        let negated = expect_variable(&ir_module.items[3], "negated");
        let negated_value = unwrap_value(
            negated
                .value
                .as_ref()
                .expect("negated should have initializer"),
        );
        match negated_value {
            IrExpression::RuntimeCall(RuntimeNamespace::Value(ValueCall::LogicalNot { expr })) => {
                match expr.as_ref() {
                    IrExpression::Identifier(name) => assert_eq!(name, "value"),
                    other => panic!("expected identifier operand for logical not, got {other:?}"),
                }
            }
            other => panic!("expected logical not runtime call, got {other:?}"),
        }

        let bitwise = expect_variable(&ir_module.items[4], "bitwise");
        match bitwise.value.as_ref().expect("bitwise should exist") {
            IrExpression::Unary { op, .. } => assert!(matches!(op, ir::IrUnaryOp::BitwiseNot)),
            other => panic!("expected bitwise unary, got {other:?}"),
        }

        let typeofed = expect_variable(&ir_module.items[5], "typeofed");
        match typeofed.value.as_ref().expect("typeofed should exist") {
            IrExpression::Unary { op, .. } => assert!(matches!(op, ir::IrUnaryOp::TypeOf)),
            other => panic!("expected typeof unary, got {other:?}"),
        }

        let voided = expect_variable(&ir_module.items[6], "voided");
        let void_expr = unwrap_value(voided.value.as_ref().expect("voided should exist"));
        match void_expr {
            IrExpression::Unary { op, .. } => assert!(matches!(op, ir::IrUnaryOp::Void)),
            other => panic!("expected void unary, got {other:?}"),
        }

        let removed = expect_variable(&ir_module.items[8], "removed");
        match removed.value.as_ref().expect("removed should exist") {
            IrExpression::Delete(target) => match target {
                IrDeleteTarget::Property { object, property } => {
                    assert_identifier(object, "record");
                    match property {
                        IrDeleteProperty::Static(name) => assert_eq!(name, "name"),
                        other => panic!("expected static property delete, got {other:?}"),
                    }
                }
                other => panic!("expected property delete target, got {other:?}"),
            },
            other => panic!("expected delete expression, got {other:?}"),
        }
    }

    #[test]
    fn lowers_parenthesized_expressions() {
        let ir_module = lower(
            r#"
            const result = (value);
        "#,
        );

        assert_eq!(ir_module.items.len(), 1);
        let variable = expect_variable(&ir_module.items[0], "result");
        let value = unwrap_value(
            variable
                .value
                .as_ref()
                .expect("result should have initializer"),
        );
        match value {
            IrExpression::Identifier(name) => assert_eq!(name, "value"),
            other => panic!("expected identifier after removing parentheses, got {other:?}"),
        }
    }
}
