use crate::test_utils::{
    assert_bool_literal, assert_identifier, assert_number_literal, assert_string_literal,
    expect_variable, lower,
};
use ir::{IrBinOp, IrExpression, IrItem, IrParam, IrStmt, IrTemplatePart, IrType};

#[test]
fn handles_literal_initializers() {
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
    assert_eq!(total.ty, IrType::Number);
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
fn infers_return_type_from_branches() {
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
    assert_eq!(
        function.params,
        vec![IrParam {
            name: "flag".into(),
            ty: IrType::Bool
        }]
    );
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

            let else_branch = else_branch
                .as_ref()
                .expect("if statement should have else branch");
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
