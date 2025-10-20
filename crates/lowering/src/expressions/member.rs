use super::*;
use ir::{ArrayCall, ConsoleCall, RuntimeNamespace};

#[derive(Debug)]
pub(crate) struct LoweredMember {
    expression: IrExpression,
    runtime: Option<MemberRuntime>,
}

impl LoweredMember {
    pub(crate) fn into_expression(self) -> IrExpression {
        self.expression
    }

    pub(crate) fn into_callee(self) -> MemberCallee {
        match self.runtime {
            Some(runtime) => MemberCallee::Runtime(runtime),
            None => MemberCallee::Expr(self.expression),
        }
    }
}

#[derive(Debug)]
pub(crate) enum MemberCallee {
    Runtime(MemberRuntime),
    Expr(IrExpression),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum MemberRuntime {
    ConsoleLog,
    ArrayPush { target: IrExpression },
}

impl MemberRuntime {
    pub(crate) fn into_runtime_call(self, args: Vec<IrExpression>) -> RuntimeNamespace {
        match self {
            MemberRuntime::ConsoleLog => RuntimeNamespace::Console(ConsoleCall::Log(args)),
            MemberRuntime::ArrayPush { target } => RuntimeNamespace::Array(ArrayCall::Push {
                target: Box::new(target),
                args,
            }),
        }
    }
}

pub(crate) fn lower_member_expr(member: &ast::MemberExpr) -> LoweredMember {
    let object = expr_to_ir(member.obj.as_ref());
    let property = match &member.prop {
        ast::MemberProp::Ident(ident) => ident.sym.to_string(),
        ast::MemberProp::PrivateName(_) => "private_not_supported".to_string(),
        ast::MemberProp::Computed(_) => "computed_not_supported".to_string(),
    };

    let runtime = detect_runtime_member(&object, &property);

    LoweredMember {
        expression: IrExpression::Member {
            object: Box::new(object),
            property,
        },
        runtime,
    }
}

fn detect_runtime_member(object: &IrExpression, property: &str) -> Option<MemberRuntime> {
    match (object, property) {
        (IrExpression::Identifier(name), "log") if name == "console" => {
            Some(MemberRuntime::ConsoleLog)
        }
        (IrExpression::Identifier(_), "push") => {
            Some(MemberRuntime::ArrayPush {
                target: object.clone(),
            })
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::lower;
    use ir::{IrExpression, IrItem};
    use swc_common::{SyntaxContext, DUMMY_SP};
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
        match lowered.runtime {
            Some(MemberRuntime::ConsoleLog) => {}
            other => panic!("expected console.log runtime, got {other:?}"),
        }

        match lowered.into_expression() {
            IrExpression::Member { object, property } => {
                assert!(matches!(
                    object.as_ref(),
                    IrExpression::Identifier(name) if name == "console"
                ));
                assert_eq!(property, "log");
            }
            other => panic!("expected member expression, got {other:?}"),
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
        match lowered.runtime {
            Some(MemberRuntime::ArrayPush { target }) => match target {
                IrExpression::Identifier(name) => assert_eq!(name, "values"),
                other => panic!("expected identifier target, got {other:?}"),
            },
            other => panic!("expected array.push runtime, got {other:?}"),
        }
    }

    #[test]
    fn lowers_plain_member_expression() {
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
}
