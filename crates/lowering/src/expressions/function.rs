use super::*;
use crate::infer::infer_function_return_type;
use crate::params::params_to_ir;
use crate::statements::block_to_ir;
use crate::types::ts_type_ann_to_ir;
use ir::{IrFunctionExpr, IrType};

pub fn function_expr_to_ir(fn_expr: &ast::FnExpr) -> IrExpression {
    if fn_expr.function.is_async {
        return IrExpression::Identifier("async_function_expression_not_supported".to_string());
    }

    if fn_expr.function.is_generator {
        return IrExpression::Identifier("generator_function_expression_not_supported".to_string());
    }

    let name = fn_expr.ident.as_ref().map(|ident| ident.sym.to_string());
    let params = params_to_ir(&fn_expr.function.params);
    let ret = fn_expr
        .function
        .return_type
        .as_ref()
        .map(|ann| ts_type_ann_to_ir(ann))
        .unwrap_or(IrType::Any);
    let body = fn_expr
        .function
        .body
        .as_ref()
        .map(block_to_ir)
        .unwrap_or_default();

    let mut ir_fn_expr = IrFunctionExpr {
        name,
        params,
        ret,
        body,
    };

    if matches!(ir_fn_expr.ret, IrType::Any) {
        if let Some(inferred) = infer_function_return_type(&ir_fn_expr.body) {
            ir_fn_expr.ret = inferred;
        }
    }

    IrExpression::Function(Box::new(ir_fn_expr))
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{assert_number_literal, assert_string_literal, expect_variable, lower};
    use ir::{IrBinOp, IrExpression, IrStmt, IrType};

    #[test]
    fn lowers_function_expression_into_ir_variant() {
        let ir_module = lower(
            r#"
            const handler = function (value: number) {
                return value + 1;
            };
        "#,
        );

        assert_eq!(ir_module.items.len(), 1);
        let handler = expect_variable(&ir_module.items[0], "handler");
        assert!(!handler.mutable);

        let function = match handler.value.as_ref() {
            Some(IrExpression::Function(function)) => function,
            other => panic!("expected function expression initializer, got {other:?}"),
        };

        assert!(function.name.is_none());
        assert_eq!(function.params.len(), 1);
        assert_eq!(function.params[0].name, "value");
        assert_eq!(function.params[0].ty, IrType::Number);

        assert_eq!(function.ret, IrType::Number);
        assert_eq!(function.body.len(), 1);

        match &function.body[0] {
            IrStmt::Return(Some(expr)) => match expr {
                IrExpression::Binary { op, .. } => assert_eq!(*op, IrBinOp::Add),
                other => panic!("expected binary addition in return, got {other:?}"),
            },
            other => panic!("expected return statement, got {other:?}"),
        }
    }

    #[test]
    fn function_expression_captures_internal_name_and_infers_return_type() {
        let ir_module = lower(
            r#"
            const handler = function internal(value: number) {
                return value * 2;
            };
        "#,
        );

        assert_eq!(ir_module.items.len(), 1);
        let variable = expect_variable(&ir_module.items[0], "handler");

        let function = match variable.value.as_ref() {
            Some(IrExpression::Function(function)) => function,
            other => panic!("expected function expression initializer, got {other:?}"),
        };

        assert_eq!(function.name.as_deref(), Some("internal"));
        assert_eq!(function.params.len(), 1);
        assert_eq!(function.params[0].name, "value");
        assert_eq!(function.params[0].ty, IrType::Number);
        assert_eq!(function.ret, IrType::Number);

        match &function.body[0] {
            IrStmt::Return(Some(expr)) => match expr {
                IrExpression::Binary { op, .. } => assert_eq!(*op, IrBinOp::Mul),
                other => panic!("expected multiplication return, got {other:?}"),
            },
            other => panic!("expected return statement, got {other:?}"),
        }
    }

    #[test]
    fn function_expression_infers_unit_when_no_return_present() {
        let ir_module = lower(
            r#"
            const noop = function () {
                const value = 1;
            };
        "#,
        );

        assert_eq!(ir_module.items.len(), 1);
        let variable = expect_variable(&ir_module.items[0], "noop");

        let function = match variable.value.as_ref() {
            Some(IrExpression::Function(function)) => function,
            other => panic!("expected function expression initializer, got {other:?}"),
        };

        assert!(function.name.is_none());
        assert!(function.params.is_empty());
        assert_eq!(function.ret, IrType::Unit);
        assert_eq!(function.body.len(), 1);

        match &function.body[0] {
            IrStmt::VarDecl(vars) => {
                assert_eq!(vars.len(), 1);
                let var = &vars[0];
                assert_eq!(var.name, "value");
                assert!(!var.mutable);
                assert_number_literal(var.value.as_ref(), 1.0);
            }
            other => panic!("expected inner const declaration, got {other:?}"),
        }
    }

    #[test]
    fn function_expression_respects_explicit_return_annotation() {
        let ir_module = lower(
            r#"
            const creator = function (): string {
                return "done";
            };
        "#,
        );

        assert_eq!(ir_module.items.len(), 1);
        let variable = expect_variable(&ir_module.items[0], "creator");

        let function = match variable.value.as_ref() {
            Some(IrExpression::Function(function)) => function,
            other => panic!("expected function expression initializer, got {other:?}"),
        };

        assert_eq!(function.ret, IrType::Str);
        assert_eq!(function.body.len(), 1);

        match &function.body[0] {
            IrStmt::Return(Some(expr)) => assert_string_literal(Some(expr), "done"),
            other => panic!("expected return statement, got {other:?}"),
        }
    }

    #[test]
    fn async_and_generator_function_expressions_are_marked_unsupported() {
        let ir_module = lower(
            r#"
            const asyncHandler = async function () {};
            const generator = function* () {};
        "#,
        );

        assert_eq!(ir_module.items.len(), 2);

        let async_var = expect_variable(&ir_module.items[0], "asyncHandler");
        match async_var.value.as_ref() {
            Some(IrExpression::Identifier(name)) => {
                assert_eq!(name, "async_function_expression_not_supported");
            }
            other => panic!("expected unsupported sentinel identifier, got {other:?}"),
        }

        let generator_var = expect_variable(&ir_module.items[1], "generator");
        match generator_var.value.as_ref() {
            Some(IrExpression::Identifier(name)) => {
                assert_eq!(name, "generator_function_expression_not_supported");
            }
            other => panic!("expected unsupported sentinel identifier, got {other:?}"),
        }
    }
}
