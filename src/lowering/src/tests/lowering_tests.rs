use crate::ast_to_ir;
use crate::expressions::call_to_ir;
use ir::{
    ConsoleCall, IrArrowBody, IrBinOp, IrExpression, IrItem, IrLiteral, IrParam, IrStmt,
    IrTemplatePart, IrType, IrVariable, RuntimeNamespace,
};
use parser::ast as parse_ts_module;
use swc_common::DUMMY_SP;
use swc_ecma_ast as swc_ast;

fn lower(source: &str) -> ir::IrModule {
    let module = parse_ts_module(source);
    ast_to_ir(&module)
}

#[test]
fn lowers_variable_declarations_with_literal_initializers() {
    let ir_module = lower(
        r#"
        let greeting: string = "hello";
        let flag: boolean = true;
        let total = 1 + 2;
        let negative = -42;
    "#,
    );

    assert_eq!(ir_module.items.len(), 4);

    let greeting = expect_variable(&ir_module.items[0], "greeting");
    assert!(greeting.mutable);
    assert_eq!(greeting.ty, IrType::Str);
    assert_string_literal(greeting.value.as_ref(), "hello");

    let flag = expect_variable(&ir_module.items[1], "flag");
    assert!(flag.mutable);
    assert_eq!(flag.ty, IrType::Bool);
    assert_bool_literal(flag.value.as_ref(), true);

    let total = expect_variable(&ir_module.items[2], "total");
    assert!(total.mutable);
    assert_eq!(total.ty, IrType::Any);
    let expr = total.value.as_ref().expect("total should have initializer");
    match expr {
        IrExpression::Binary { op, left, right } => {
            assert_eq!(*op, IrBinOp::Add);
            assert_number_literal(Some(left), 1.0);
            assert_number_literal(Some(right), 2.0);
        }
        other => panic!("expected binary expression for total, got {other:?}"),
    }

    let negative = expect_variable(&ir_module.items[3], "negative");
    assert!(negative.mutable);
    assert_number_literal(negative.value.as_ref(), -42.0);
}

#[test]
fn lowers_function_with_control_flow_and_inferred_return() {
    let ir_module = lower(
        r#"
        function choose(flag: boolean) {
            const label: string = "maybe";

            if (flag) {
                return "yes";
            } else {
                return `no`;
            }

            return "fallback";
        }
    "#,
    );

    assert_eq!(ir_module.items.len(), 1);
    let function = match &ir_module.items[0] {
        IrItem::Function(func) => func,
        other => panic!("expected function IR item, got {other:?}"),
    };

    assert_eq!(function.name, "choose");
    assert_eq!(function.params, vec![IrParam { name: "flag".into(), ty: IrType::Bool }]);
    assert_eq!(function.ret, IrType::Str);
    assert_eq!(function.body.len(), 3);

    match &function.body[0] {
        IrStmt::VarDecl(vars) => {
            assert_eq!(vars.len(), 1);
            let var = &vars[0];
            assert_eq!(var.name, "label");
            assert!(!var.mutable);
            assert_eq!(var.ty, IrType::Str);
            assert_string_literal(var.value.as_ref(), "maybe");
        }
        other => panic!("expected leading const declaration, got {other:?}"),
    }

    match &function.body[1] {
        IrStmt::If {
            condition,
            then_branch,
            else_branch,
        } => {
            assert_identifier(condition, "flag");

            assert_eq!(then_branch.len(), 1);
            match &then_branch[0] {
                IrStmt::Return(Some(expr)) => assert_string_literal(Some(expr), "yes"),
                other => panic!("expected return in then branch, got {other:?}"),
            }

            let else_branch = else_branch.as_ref().expect("if statement should have else branch");
            assert_eq!(else_branch.len(), 1);
            match &else_branch[0] {
                IrStmt::Return(Some(expr)) => match expr {
                    IrExpression::Template(parts) => {
                        assert_eq!(parts.len(), 1);
                        assert!(matches!(parts[0], IrTemplatePart::String(ref s) if s == "no"));
                    }
                    other => panic!("expected template string in else return, got {other:?}"),
                },
                other => panic!("expected return in else branch, got {other:?}"),
            }
        }
        other => panic!("expected trailing if statement, got {other:?}"),
    }

    match &function.body[2] {
        IrStmt::Return(Some(expr)) => assert_string_literal(Some(expr), "fallback"),
        other => panic!("expected final return statement, got {other:?}"),
    }
}

#[test]
fn lowers_console_log_expression_to_runtime_call() {
    let ir_module = lower("console.log('answer', 42, true);");

    assert_eq!(ir_module.items.len(), 1);
    match &ir_module.items[0] {
        IrItem::Expression(expr) => match expr {
            IrExpression::RuntimeCall(RuntimeNamespace::Console(ConsoleCall::Log(args))) => {
                assert_eq!(args.len(), 3);
                assert_string_literal(Some(&args[0]), "answer");
                assert_number_literal(Some(&args[1]), 42.0);
                assert_bool_literal(Some(&args[2]), true);
            }
            other => panic!("expected runtime console log, got {other:?}"),
        },
        other => panic!("expected expression item, got {other:?}"),
    }
}

#[test]
fn lowers_block_statement_into_ir_block() {
    let ir_module = lower("{ let value: number = 5; value; }");

    assert_eq!(ir_module.items.len(), 1);
    let block_stmts = match &ir_module.items[0] {
        IrItem::Block(stmts) => stmts,
        other => panic!("expected block item, got {other:?}"),
    };

    assert_eq!(block_stmts.len(), 2);
    match &block_stmts[0] {
        IrStmt::VarDecl(vars) => {
            assert_eq!(vars.len(), 1);
            let var = &vars[0];
            assert_eq!(var.name, "value");
            assert!(var.mutable);
            assert_eq!(var.ty, IrType::Number);
            assert_number_literal(var.value.as_ref(), 5.0);
        }
        other => panic!("expected variable declaration inside block, got {other:?}"),
    }

    match &block_stmts[1] {
        IrStmt::Expression(expr) => assert_identifier(expr, "value"),
        other => panic!("expected trailing expression, got {other:?}"),
    }
}

#[test]
fn lowers_arrow_functions_with_expression_and_block_bodies() {
    let ir_module = lower(
        r#"
        const double = (value: number) => value * 2;
        const format = (value: string, ...rest: number[]) => {
            return `value:${value}`;
        };
    "#,
    );

    assert_eq!(ir_module.items.len(), 2);

    let double = expect_variable(&ir_module.items[0], "double");
    assert!(!double.mutable);
    let arrow = match double.value.as_ref().expect("double should have initializer") {
        IrExpression::Arrow { params, body } => {
            assert_eq!(params, &vec![IrParam { name: "value".into(), ty: IrType::Number }]);
            body
        }
        other => panic!("expected arrow expression for double, got {other:?}"),
    };
    match arrow {
        IrArrowBody::Expr(expr) => match expr.as_ref() {
            IrExpression::Binary { op, .. } => assert_eq!(*op, IrBinOp::Mul),
            other => panic!("expected multiplication in arrow body, got {other:?}"),
        },
        other => panic!("expected expression body, got {other:?}"),
    }

    let format = expect_variable(&ir_module.items[1], "format");
    assert!(!format.mutable);
    let arrow = match format.value.as_ref().expect("format should have initializer") {
        IrExpression::Arrow { params, body } => {
            assert_eq!(params.len(), 2);
            assert_eq!(params[0], IrParam { name: "value".into(), ty: IrType::Str });
            assert_eq!(params[1].name, "rest");
            assert_eq!(params[1].ty, IrType::Any);
            body
        }
        other => panic!("expected arrow expression for format, got {other:?}"),
    };

    let block = match arrow {
        IrArrowBody::Block(stmts) => stmts,
        other => panic!("expected block arrow body, got {other:?}"),
    };

    assert_eq!(block.len(), 1);
    match &block[0] {
        IrStmt::Return(Some(expr)) => match expr {
            IrExpression::Template(parts) => {
                assert_eq!(parts.len(), 3);
                assert!(matches!(parts[0], IrTemplatePart::String(ref s) if s == "value:"));
                match &parts[1] {
                    IrTemplatePart::Expr(inner) => assert_identifier(inner, "value"),
                    other => panic!("expected interpolation expression, got {other:?}"),
                }
                assert!(matches!(parts[2], IrTemplatePart::String(ref s) if s.is_empty()));
            }
            other => panic!("expected template literal in return, got {other:?}"),
        },
        other => panic!("expected return statement in block arrow, got {other:?}"),
    }
}

#[test]
fn lowers_arrays_conditionals_and_member_expressions() {
    let ir_module = lower(
        r#"
        const numbers = [1, 2, 3];
        let result = flag ? numbers : [0];
        foo.bar(1);
    "#,
    );

    assert_eq!(ir_module.items.len(), 3);

    let numbers = expect_variable(&ir_module.items[0], "numbers");
    assert!(!numbers.mutable);
    let array = match numbers.value.as_ref().expect("numbers should have initializer") {
        IrExpression::Array(elements) => elements,
        other => panic!("expected array literal, got {other:?}"),
    };
    assert_eq!(array.len(), 3);
    assert_number_literal(Some(&array[0]), 1.0);
    assert_number_literal(Some(&array[1]), 2.0);
    assert_number_literal(Some(&array[2]), 3.0);

    let result = expect_variable(&ir_module.items[1], "result");
    assert!(result.mutable);
    match result.value.as_ref().expect("result should have initializer") {
        IrExpression::Conditional { test, consequent, alternate } => {
            assert_identifier(test, "flag");
            assert_identifier(consequent, "numbers");
            match alternate.as_ref() {
                IrExpression::Array(elements) => {
                    assert_eq!(elements.len(), 1);
                    assert_number_literal(Some(&elements[0]), 0.0);
                }
                other => panic!("expected array literal in alternate branch, got {other:?}"),
            }
        }
        other => panic!("expected conditional expression, got {other:?}"),
    }

    match &ir_module.items[2] {
        IrItem::Expression(expr) => match expr {
            IrExpression::Call { callee, args } => {
                match callee.as_ref() {
                    IrExpression::Member { object, property } => {
                        assert_identifier(object, "foo");
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
fn call_to_ir_maps_super_calls() {
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

fn expect_variable<'a>(item: &'a IrItem, name: &str) -> &'a IrVariable {
    match item {
        IrItem::Variable(var) if var.name == name => var,
        other => panic!("expected variable {name}, got {other:?}"),
    }
}

fn assert_identifier(expr: &IrExpression, expected: &str) {
    match expr {
        IrExpression::Identifier(name) => assert_eq!(name, expected),
        other => panic!("expected identifier {expected}, got {other:?}"),
    }
}

fn assert_number_literal(expr: Option<&IrExpression>, expected: f64) {
    let expr = expr.expect("expected number literal expression");
    match expr {
        IrExpression::Literal(IrLiteral::Number(value)) => {
            assert!((value - expected).abs() < f64::EPSILON, "expected {expected}, got {value}");
        }
        other => panic!("expected numeric literal {expected}, got {other:?}"),
    }
}

fn assert_string_literal(expr: Option<&IrExpression>, expected: &str) {
    let expr = expr.expect("expected string literal expression");
    match expr {
        IrExpression::Literal(IrLiteral::Str(value)) => assert_eq!(value, expected),
        other => panic!("expected string literal {expected}, got {other:?}"),
    }
}

fn assert_bool_literal(expr: Option<&IrExpression>, expected: bool) {
    let expr = expr.expect("expected bool literal expression");
    match expr {
        IrExpression::Literal(IrLiteral::Bool(value)) => assert_eq!(*value, expected),
        other => panic!("expected bool literal {expected}, got {other:?}"),
    }
}
