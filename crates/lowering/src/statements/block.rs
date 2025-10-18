use ir::IrStmt;
use swc_ecma_ast::{self as ast};

pub(crate) fn block_to_ir(block: &ast::BlockStmt) -> Vec<IrStmt> {
    block.stmts.iter().map(super::stmt_to_ir).collect()
}

pub(crate) fn from_block(block: &ast::BlockStmt) -> IrStmt {
    IrStmt::Block(block_to_ir(block))
}
