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
        return IrExpression::Identifier(
            "generator_function_expression_not_supported".to_string(),
        );
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
