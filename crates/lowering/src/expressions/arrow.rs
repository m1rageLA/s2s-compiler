use super::*;
use crate::context;
use crate::params::params_to_ir;
use crate::statements::block_to_ir;
use ir::IrType;
use swc_ecma_ast::BlockStmtOrExpr;

pub fn arrow_expr_to_ir(arrow: &ast::ArrowExpr) -> IrExpression {
    let params = params_to_ir(&arrow.params);
    context::push_scope();
    for param in &params {
        context::define(&param.name, param.ty);
    }
    context::push_function_return(IrType::Any);
    let body = match &*arrow.body {
        BlockStmtOrExpr::Expr(expr) => IrArrowBody::Expr(Box::new(expr_to_ir(expr))),
        BlockStmtOrExpr::BlockStmt(block) => IrArrowBody::Block(block_to_ir(block)),
    };
    context::pop_function_return();
    context::pop_scope();

    IrExpression::Arrow { params, body }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{assert_identifier, expect_variable, lower};
    use ir::{IrExpression, IrParam, IrStmt, IrTemplatePart, IrType};

    #[test]
    fn lowers_arrow_expression_bodies() {
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
        let function = match double
            .value
            .as_ref()
            .expect("double should have initializer")
        {
            IrExpression::RuntimeCall(ir::RuntimeNamespace::Value(ir::ValueCall::Coerce {
                expr,
            })) => match expr.as_ref() {
                IrExpression::Function(func) => func,
                other => panic!("expected function expression for double, got {other:?}"),
            },
            IrExpression::Function(func) => func,
            other => panic!("expected function expression for double, got {other:?}"),
        };

        assert_eq!(
            function.params,
            vec![IrParam {
                name: "value".into(),
                ty: IrType::Number
            }]
        );
        match function.body.as_slice() {
            [IrStmt::Return(Some(expr))] => {
                let expr = match expr {
                    IrExpression::RuntimeCall(ir::RuntimeNamespace::Value(
                        ir::ValueCall::Coerce { expr },
                    )) => expr.as_ref(),
                    other => other,
                };
                match expr {
                    IrExpression::RuntimeCall(ir::RuntimeNamespace::Value(
                        ir::ValueCall::Mul { .. },
                    )) => {}
                    other => panic!("expected multiplicative return, got {other:?}"),
                }
            }
            other => panic!("expected single return statement, got {other:?}"),
        }

        let format = expect_variable(&ir_module.items[1], "format");
        assert!(!format.mutable);
        let function = match format
            .value
            .as_ref()
            .expect("format should have initializer")
        {
            IrExpression::RuntimeCall(ir::RuntimeNamespace::Value(ir::ValueCall::Coerce {
                expr,
            })) => match expr.as_ref() {
                IrExpression::Function(func) => func,
                other => panic!("expected function expression for format, got {other:?}"),
            },
            IrExpression::Function(func) => func,
            other => panic!("expected function expression for format, got {other:?}"),
        };

        assert_eq!(function.params.len(), 2);
        assert_eq!(
            function.params[0],
            IrParam {
                name: "value".into(),
                ty: IrType::Str
            }
        );
        assert_eq!(function.params[1].name, "rest");
        assert_eq!(function.params[1].ty, IrType::Any);

        assert_eq!(function.body.len(), 1);
        match &function.body[0] {
            IrStmt::Return(Some(expr)) => {
                let expr = match expr {
                    IrExpression::RuntimeCall(ir::RuntimeNamespace::Value(
                        ir::ValueCall::Coerce { expr },
                    )) => expr.as_ref(),
                    other => other,
                };

                match expr {
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
                }
            }
            other => panic!("expected return statement in block arrow, got {other:?}"),
        }
    }
}
