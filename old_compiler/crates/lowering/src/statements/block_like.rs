use ir::IrStmt;
use swc_ecma_ast::{self as ast};

pub(crate) fn stmt_block_like_to_ir(stmt: &ast::Stmt) -> Vec<IrStmt> {
    match stmt {
        ast::Stmt::Block(block) => super::block::block_to_ir(block),
        other => vec![super::stmt_to_ir(other)],
    }
}
