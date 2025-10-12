use super::*;
use crate::params::params_to_ir;
use crate::statements::block_to_ir;
use swc_ecma_ast::BlockStmtOrExpr;

pub fn arrow_expr_to_ir(arrow: &ast::ArrowExpr) -> IrExpression {
    let params = params_to_ir(&arrow.params);
    let body = match &*arrow.body {
        BlockStmtOrExpr::Expr(expr) => IrArrowBody::Expr(Box::new(expr_to_ir(expr))),
        BlockStmtOrExpr::BlockStmt(block) => IrArrowBody::Block(block_to_ir(block)),
    };

    IrExpression::Arrow { params, body }
}
    
