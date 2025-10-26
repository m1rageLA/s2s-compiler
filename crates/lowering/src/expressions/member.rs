use super::*;
use crate::context;
use ir::{ArrayCall, ConsoleCall, IrArrayKind, IrExpression, IrType, RuntimeNamespace};

pub(crate) fn lower_member_expr(member: &ast::MemberExpr) -> IrExpression {
    let object = expr_to_ir(member.obj.as_ref());

    match &member.prop {
        ast::MemberProp::Ident(ident) => IrExpression::Member {
            object: Box::new(object),
            property: ident.sym.to_string(),
        },
        ast::MemberProp::PrivateName(_) => IrExpression::Member {
            object: Box::new(object),
            property: "private_not_supported".to_string(),
        },
        ast::MemberProp::Computed(expr) => lower_computed_member(object, expr.expr.as_ref()),
    }
}

pub(crate) fn runtime_call_for_member(
    callee: &IrExpression,
    args: &[IrExpression],
) -> Option<RuntimeNamespace> {
    let IrExpression::Member { object, property } = callee else {
        return None;
    };

    detect_runtime_call(object.as_ref(), property, args)
}

fn detect_runtime_call(
    object: &IrExpression,
    property: &str,
    args: &[IrExpression],
) -> Option<RuntimeNamespace> {
    match (object, property) {
        (IrExpression::Identifier(name), "log") if name == "console" => {
            Some(RuntimeNamespace::Console(ConsoleCall::Log(args.to_vec())))
        }
        (IrExpression::Identifier(_), "push") => {
            let coerced_args = args
                .iter()
                .cloned()
                .map(coerce_to_value)
                .collect::<Vec<_>>();
            Some(RuntimeNamespace::Array(ArrayCall::Push {
                target: Box::new(object.clone()),
                args: coerced_args,
            }))
        }
        (_, "map") => args.first().cloned().map(|callback| {
            RuntimeNamespace::Array(ArrayCall::Map {
                target: Box::new(object.clone()),
                callback: Box::new(callback),
            })
        }),
        (_, "filter") => args.first().cloned().map(|callback| {
            RuntimeNamespace::Array(ArrayCall::Filter {
                target: Box::new(object.clone()),
                callback: Box::new(callback),
            })
        }),
        _ => None,
    }
}

pub(crate) fn runtime_value_for_member(member: &IrExpression) -> Option<IrExpression> {
    let IrExpression::Member { object, property } = member else {
        return None;
    };

    match (object.as_ref(), property.as_str()) {
        (IrExpression::Identifier(_), "length") => Some(IrExpression::RuntimeCall(
            RuntimeNamespace::Array(ArrayCall::Length {
                target: Box::new(object.as_ref().clone()),
            }),
        )),
        _ => None,
    }
}

fn lower_computed_member(object: IrExpression, property: &ast::Expr) -> IrExpression {
    let property_ir = expr_to_ir(property);

    if let IrExpression::Literal(IrLiteral::Str(name)) = property_ir.clone() {
        return IrExpression::Member {
            object: Box::new(object),
            property: name,
        };
    }

    let element_kind = infer_array_kind(&object);

    IrExpression::RuntimeCall(RuntimeNamespace::Array(ArrayCall::Index {
        target: Box::new(object),
        index: Box::new(property_ir),
        element: element_kind,
    }))
}

fn infer_array_kind(expr: &IrExpression) -> Option<IrArrayKind> {
    match expr {
        IrExpression::Identifier(name) => match context::lookup(name) {
            Some(IrType::Array(kind)) => Some(kind),
            _ => None,
        },
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::lower;
    use ir::{IrExpression, IrItem};
    use swc_common::{DUMMY_SP, SyntaxContext};
    use swc_ecma_ast as swc_ast;

    fn lower_expression(source: &str) -> IrExpression {
        let ir_module = lower(&format!("{source};"));
        assert_eq!(ir_module.items.len(), 1, "expected single expression");
        match ir_module.items.into_iter().next().expect("expression item") {
            IrItem::Expression(expr) => expr,
            other => panic!("expected expression item, got {other:?}"),
        }
    }

    #[test]
    fn lowers_member_expression() {
        let ir_expr = lower_expression("foo.bar");

        match ir_expr {
            IrExpression::Member { object, property } => {
                assert!(matches!(
                    object.as_ref(),
                    IrExpression::Identifier(name) if name == "foo"
                ));
                assert_eq!(property, "bar");
            }
            other => panic!("expected member expression, got {other:?}"),
        }
    }

    #[test]
    fn detects_runtime_console_log_member() {
        let member = swc_ast::MemberExpr {
            span: DUMMY_SP,
            obj: Box::new(swc_ast::Expr::Ident(swc_ast::Ident::new(
                "console".into(),
                DUMMY_SP,
                SyntaxContext::empty(),
            ))),
            prop: swc_ast::MemberProp::Ident(
                swc_ast::Ident::new("log".into(), DUMMY_SP, SyntaxContext::empty()).into(),
            ),
        };

        let lowered = lower_member_expr(&member);

        match lowered {
            IrExpression::Member {
                ref object,
                ref property,
            } => {
                assert!(
                    matches!(object.as_ref(), IrExpression::Identifier(name) if name == "console")
                );
                assert_eq!(property, "log");
            }
            other => panic!("expected member expression, got {other:?}"),
        }

        match runtime_call_for_member(&lowered, &[]) {
            Some(RuntimeNamespace::Console(ConsoleCall::Log(args))) => assert!(args.is_empty()),
            other => panic!("expected console.log runtime, got {other:?}"),
        }
    }

    #[test]
    fn detects_array_push_runtime_member() {
        let member = swc_ast::MemberExpr {
            span: DUMMY_SP,
            obj: Box::new(swc_ast::Expr::Ident(swc_ast::Ident::new(
                "values".into(),
                DUMMY_SP,
                SyntaxContext::empty(),
            ))),
            prop: swc_ast::MemberProp::Ident(
                swc_ast::Ident::new("push".into(), DUMMY_SP, SyntaxContext::empty()).into(),
            ),
        };

        let lowered = lower_member_expr(&member);

        match runtime_call_for_member(&lowered, &[]) {
            Some(RuntimeNamespace::Array(ArrayCall::Push { target, args })) => {
                match target.as_ref() {
                    IrExpression::Identifier(name) => assert_eq!(name, "values"),
                    other => panic!("expected identifier target, got {other:?}"),
                }
                assert!(args.is_empty());
            }
            other => panic!("expected array.push runtime, got {other:?}"),
        }
    }

    #[test]
    fn lowers_array_length_to_runtime_call() {
        let ir_expr = lower_expression("values.length");

        match ir_expr {
            IrExpression::RuntimeCall(RuntimeNamespace::Array(ArrayCall::Length { target })) => {
                match target.as_ref() {
                    IrExpression::Identifier(name) => assert_eq!(name, "values"),
                    other => panic!("expected identifier target, got {other:?}"),
                }
            }
            other => panic!("expected runtime length call, got {other:?}"),
        }
    }

    #[test]
    fn lowers_array_index_with_numeric_literal() {
        let ir_expr = lower_expression("values[0]");

        match ir_expr {
            IrExpression::RuntimeCall(RuntimeNamespace::Array(ArrayCall::Index {
                target,
                index,
                element,
            })) => {
                assert!(matches!(
                    target.as_ref(),
                    IrExpression::Identifier(name) if name == "values"
                ));
                assert!(matches!(
                    index.as_ref(),
                    IrExpression::Literal(IrLiteral::Number(value)) if (*value - 0.0).abs() < f64::EPSILON
                ));
                assert!(element.is_none());
            }
            other => panic!("expected runtime index call, got {other:?}"),
        }
    }

    #[test]
    fn lowers_array_index_with_identifier_expression() {
        let ir_expr = lower_expression("values[i]");

        match ir_expr {
            IrExpression::RuntimeCall(RuntimeNamespace::Array(ArrayCall::Index {
                target,
                index,
                element,
            })) => {
                assert!(matches!(
                    target.as_ref(),
                    IrExpression::Identifier(name) if name == "values"
                ));
                assert!(matches!(
                    index.as_ref(),
                    IrExpression::Identifier(name) if name == "i"
                ));
                assert!(element.is_none());
            }
            other => panic!("expected runtime index call, got {other:?}"),
        }
    }

    #[test]
    fn computed_string_literal_maps_to_named_property() {
        let ir_expr = lower_expression(r#"values["length"]"#);

        match ir_expr {
            IrExpression::RuntimeCall(RuntimeNamespace::Array(ArrayCall::Length { target })) => {
                assert!(matches!(
                    target.as_ref(),
                    IrExpression::Identifier(name) if name == "values"
                ));
            }
            other => panic!("expected runtime length call, got {other:?}"),
        }
    }

    #[test]
    fn detects_array_map_runtime_member() {
        let lowered = lower_expression("items.map(transform)");

        match lowered {
            IrExpression::RuntimeCall(RuntimeNamespace::Array(ArrayCall::Map { target, callback })) => {
                assert!(matches!(target.as_ref(), IrExpression::Identifier(name) if name == "items"));
                assert!(matches!(callback.as_ref(), IrExpression::Identifier(name) if name == "transform"));
            }
            other => panic!("expected array.map runtime call, got {other:?}"),
        }
    }

    #[test]
    fn detects_array_filter_runtime_member() {
        let lowered = lower_expression("items.filter(predicate)");

        match lowered {
            IrExpression::RuntimeCall(RuntimeNamespace::Array(ArrayCall::Filter { target, callback })) => {
                assert!(matches!(target.as_ref(), IrExpression::Identifier(name) if name == "items"));
                assert!(matches!(callback.as_ref(), IrExpression::Identifier(name) if name == "predicate"));
            }
            other => panic!("expected array.filter runtime call, got {other:?}"),
        }
    }
}
