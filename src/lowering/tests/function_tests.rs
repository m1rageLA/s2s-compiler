mod helpers;
use helpers::{assert_identifier, assert_string_literal, lower};
use ir::{IrExpression, IrItem, IrParam, IrStmt, IrTemplatePart, IrType};

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
