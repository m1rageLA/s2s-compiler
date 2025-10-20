use super::member::{self, MemberCallee};
use super::*;

pub fn call_to_ir(c: &ast::CallExpr) -> IrExpression {
    match &c.callee {
        ast::Callee::Expr(expr) => {
            let callee = callee_to_ir(expr);
            let args = c
                .args
                .iter()
                .map(|a| match a {
                    ast::ExprOrSpread { spread: None, expr } => expr_to_ir(expr),
                    _ => IrExpression::Identifier("spread_not_supported".to_string()),
                })
                .collect::<Vec<_>>();

            match callee {
                MemberCallee::Runtime(runtime) => {
                    IrExpression::RuntimeCall(runtime.into_runtime_call(args))
                }
                MemberCallee::Expr(callee_expr) => IrExpression::Call {
                    callee: Box::new(callee_expr),
                    args,
                },
            }
        }

        ast::Callee::Super(_) => {
            let args = c
                .args
                .iter()
                .map(|a| match a {
                    ast::ExprOrSpread { spread: None, expr } => expr_to_ir(expr),
                    _ => IrExpression::Identifier("spread_not_supported".to_string()),
                })
                .collect::<Vec<_>>();

            IrExpression::SuperCall { args }
        }

        ast::Callee::Import(_) => IrExpression::Identifier("import_call_not_supported".to_string()),
    }
}

fn callee_to_ir(expr: &ast::Expr) -> MemberCallee {
    match expr {
        ast::Expr::Ident(i) => MemberCallee::Expr(IrExpression::Identifier(i.sym.to_string())),
        ast::Expr::Member(m) => member::lower_member_expr(m).into_callee(),
        ast::Expr::SuperProp(prop) => {
            let property = match &prop.prop {
                ast::SuperProp::Ident(id) => id.sym.to_string(),
                ast::SuperProp::Computed(_) => "computed_not_supported".to_string(),
            };
            MemberCallee::Expr(IrExpression::Member {
                object: Box::new(IrExpression::Identifier("super".to_string())),
                property,
            })
        }
        _ => MemberCallee::Expr(IrExpression::Identifier("unsupported".to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{assert_number_literal, lower};
    use ir::{IrExpression, IrItem, RuntimeNamespace};
    use swc_common::DUMMY_SP;
    use swc_ecma_ast as swc_ast;

    #[test]
    fn handles_super_calls() {
        let call_expr = swc_ast::CallExpr {
            span: DUMMY_SP,
            ctxt: Default::default(),
            type_args: None,
            callee: swc_ast::Callee::Super(swc_ast::Super { span: DUMMY_SP }),
            args: vec![swc_ast::ExprOrSpread {
                spread: None,
                expr: Box::new(swc_ast::Expr::Lit(swc_ast::Lit::Num(swc_ast::Number {
                    span: DUMMY_SP,
                    value: 1.0,
                    raw: None,
                }))),
            }],
        };

        match call_to_ir(&call_expr) {
            IrExpression::SuperCall { args } => {
                assert_eq!(args.len(), 1);
                assert_number_literal(Some(&args[0]), 1.0);
            }
            other => panic!("expected super call expression, got {other:?}"),
        }
    }

    #[test]
    fn lowers_member_call_expressions() {
        let ir_module = lower(
            r#"
            foo.bar(1);
        "#,
        );

        assert_eq!(ir_module.items.len(), 1);
        match &ir_module.items[0] {
            IrItem::Expression(expr) => match expr {
                IrExpression::Call { callee, args } => {
                    match callee.as_ref() {
                        IrExpression::Member { object, property } => {
                            assert!(
                                matches!(object.as_ref(), IrExpression::Identifier(name) if name == "foo")
                            );
                            assert_eq!(property, "bar");
                        }
                        other => panic!("expected member call target, got {other:?}"),
                    }
                    assert_eq!(args.len(), 1);
                    assert_number_literal(Some(&args[0]), 1.0);
                }
                other => panic!("expected call expression, got {other:?}"),
            },
            other => panic!("expected expression item, got {other:?}"),
        }
    }

    #[test]
    fn lowers_console_log_to_runtime_call() {
        let ir_module = lower(
            r#"
            console.log("debug", 1);
        "#,
        );

        assert_eq!(ir_module.items.len(), 1);
        match &ir_module.items[0] {
            IrItem::Expression(expr) => match expr {
                IrExpression::RuntimeCall(RuntimeNamespace::Console(console_call)) => match console_call {
                    ir::ConsoleCall::Log(args) => {
                        assert_eq!(args.len(), 2);
                        crate::test_utils::assert_string_literal(Some(&args[0]), "debug");
                        assert_number_literal(Some(&args[1]), 1.0);
                    }
                },
                other => panic!("expected console runtime call, got {other:?}"),
            },
            other => panic!("expected expression item, got {other:?}"),
        }
    }

    #[test]
    fn lowers_array_push_to_runtime_call() {
        let ir_module = lower(
            r#"
            let array = [1];
            array.push(2);
        "#,
        );

        assert_eq!(ir_module.items.len(), 2);
        match &ir_module.items[1] {
            IrItem::Expression(expr) => match expr {
                IrExpression::RuntimeCall(RuntimeNamespace::Array(array_call)) => match array_call {
                    ir::ArrayCall::Push { target, args } => {
                        assert!(matches!(
                            target.as_ref(),
                            IrExpression::Identifier(name) if name == "array"
                        ));
                        assert_eq!(args.len(), 1);
                        assert_number_literal(Some(&args[0]), 2.0);
                    }
                },
                other => panic!("expected array runtime call, got {other:?}"),
            },
            other => panic!("expected expression item, got {other:?}"),
        }
    }

    #[test]
    fn marks_dynamic_import_calls_as_unsupported() {
        let ir_module = lower(
            r#"
            import("mod");
        "#,
        );

        assert_eq!(ir_module.items.len(), 1);
        match &ir_module.items[0] {
            IrItem::Expression(expr) => match expr {
                IrExpression::Identifier(name) => assert_eq!(name, "import_call_not_supported"),
                other => panic!("expected unsupported identifier, got {other:?}"),
            },
            other => panic!("expected expression item, got {other:?}"),
        }
    }
}
