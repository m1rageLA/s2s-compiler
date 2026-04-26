use ir::IrItem;
use swc_ecma_ast::{self as ast};

use crate::statements::block_to_ir;

pub(crate) fn lower(block: &ast::BlockStmt, items: &mut Vec<IrItem>) {
    let ir_block = block_to_ir(block);
    items.push(IrItem::Block(ir_block));
}
