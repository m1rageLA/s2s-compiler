use super::*;
use crate::context;
use crate::params::params_to_ir;
use crate::statements::block_to_ir;
use crate::types::ts_type_ann_to_ir;
use ir::IrType;
use swc_ecma_ast::BlockStmtOrExpr;

pub fn arrow_expr_to_ir(arrow: &ast::ArrowExpr) -> IrExpression {
    let params = params_to_ir(&arrow.params);
    let ret_hint = arrow
        .return_type
        .as_ref()
        .map(|ann| ts_type_ann_to_ir(ann))
        .unwrap_or(IrType::Unit);
    context::push_scope();
    for param in &params {
        context::define(&param.name, param.ty);
    }
    context::push_function_return(ret_hint);
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
    use ir::{IrArrowBody, IrBinOp, IrExpression, IrParam, IrStmt, IrTemplatePart, IrType};

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
        let (params, body) = match double
            .value
            .as_ref()
            .expect("double should have initializer")
        {
            IrExpression::Arrow { params, body } => (params.clone(), body.clone()),
            other => panic!("expected arrow expression for double, got {other:?}"),
        };

        assert_eq!(
            params,
            vec![IrParam {
                name: "value".into(),
                ty: IrType::Number
            }]
        );
        match body {
            IrArrowBody::Expr(expr) => match expr.as_ref() {
                IrExpression::Binary {
                    op: IrBinOp::Mul, ..
                } => {}
                other => panic!("expected multiplicative return, got {other:?}"),
            },
            other => panic!("expected expression arrow body, got {other:?}"),
        }

        let format = expect_variable(&ir_module.items[1], "format");
        assert!(!format.mutable);
        let (params, body) = match format
            .value
            .as_ref()
            .expect("format should have initializer")
        {
            IrExpression::Arrow { params, body } => (params.clone(), body.clone()),
            IrExpression::Function(func) => {
                (func.params.clone(), IrArrowBody::Block(func.body.clone()))
            }
            other => panic!("expected arrow or function expression for format, got {other:?}"),
        };

        assert_eq!(params.len(), 2);
        assert_eq!(
            params[0],
            IrParam {
                name: "value".into(),
                ty: IrType::Str
            }
        );
        assert_eq!(params[1].name, "rest");
        assert_eq!(params[1].ty, IrType::Any);

        match body {
            IrArrowBody::Block(stmts) => {
                assert_eq!(stmts.len(), 1);
                match &stmts[0] {
                    IrStmt::Return(Some(expr)) => {
                        let inner = match expr {
                            IrExpression::RuntimeCall(ir::RuntimeNamespace::Value(
                                ir::ValueCall::Coerce { expr },
                            )) => expr.as_ref(),
                            other => other,
                        };
                        match inner {
                            IrExpression::Template(parts) => {
                                assert_eq!(parts.len(), 3);
                                assert!(
                                    matches!(parts[0], IrTemplatePart::String(ref s) if s == "value:")
                                );
                                match &parts[1] {
                                    IrTemplatePart::Expr(inner) => {
                                        assert_identifier(inner, "value")
                                    }
                                    other => {
                                        panic!("expected interpolation expression, got {other:?}")
                                    }
                                }
                                assert!(
                                    matches!(parts[2], IrTemplatePart::String(ref s) if s.is_empty())
                                );
                            }
                            other => panic!("expected template literal in return, got {other:?}"),
                        }
                    }
                    other => panic!("expected return statement in block arrow, got {other:?}"),
                }
            }
            other => panic!("expected block arrow body, got {other:?}"),
        }
    }
}
