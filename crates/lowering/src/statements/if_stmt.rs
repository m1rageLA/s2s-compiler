use ir::IrStmt;
use swc_ecma_ast::{self as ast};

use crate::expressions::expr_to_ir;

pub(crate) fn lower(if_stmt: &ast::IfStmt) -> IrStmt {
    let condition = expr_to_ir(&if_stmt.test);
    let then_branch = super::stmt_block_like_to_ir(&if_stmt.cons);
    let else_branch = if_stmt
        .alt
        .as_ref()
        .map(|alt| super::stmt_block_like_to_ir(alt));
    IrStmt::If {
        condition,
        then_branch,
        else_branch,
    }
}
