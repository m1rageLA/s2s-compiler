use ir::{IrExpression, IrLiteral, IrStmt, IrType, RuntimeNamespace};
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
    match context::current_function_return() {
        // Coerce when the function's declared return is represented by the runtime Value.
        // - For `Value`, everything should flow through the runtime unless it's already
        //   expressed as a runtime value call.
        Some(IrType::Value) => {
            !matches!(expr, IrExpression::RuntimeCall(RuntimeNamespace::Value(_)))
        }
        // - For `Str`, allow literal/template expressions to remain as-is (they already emit
        //   runtime values in codegen) while still coercing everything else.
        Some(IrType::Str) => match expr {
            IrExpression::RuntimeCall(RuntimeNamespace::Value(_))
            | IrExpression::Literal(IrLiteral::Str(_))
            | IrExpression::Template(_) => false,
            _ => true,
        },
        _ => false,
    }
}
