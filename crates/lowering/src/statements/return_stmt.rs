use ir::{IrExpression, IrStmt, IrType};
use swc_ecma_ast::{self as ast};

use crate::context;
use crate::expressions::{coerce_to_value, expr_to_ir};
use crate::infer;

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
        Some(IrType::Any | IrType::Value) => match infer::infer_expression_type(expr) {
            Some(IrType::Number | IrType::Str | IrType::Bool | IrType::Unit) => false,
            Some(IrType::Value) => false,
            _ => true,
        },
        _ => false,
    }
}
