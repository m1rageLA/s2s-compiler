use ir::{IrExpression, IrStmt, IrType, RuntimeNamespace};
use swc_ecma_ast::{self as ast};

use crate::context;
use crate::expressions::{coerce_to_value, expr_to_ir};
// infer is no longer needed here; type decisions are made by inspecting the
// lowered expression shape rather than running inference.

pub(crate) fn lower(ret_stmt: &ast::ReturnStmt) -> IrStmt {
    let value = ret_stmt.arg.as_ref().map(|expr| {
        let ir_expr = expr_to_ir(expr);
        if should_coerce_return(&ir_expr) {
            coerce_to_value(ir_expr)
        } else {
            ir_expr
        }
    });

    IrStmt::Return(value)
}

fn should_coerce_return(expr: &IrExpression) -> bool {
    matches!(context::current_function_return(), Some(IrType::Value | IrType::Any))
        && !matches!(expr, IrExpression::RuntimeCall(RuntimeNamespace::Value(_)))
}
