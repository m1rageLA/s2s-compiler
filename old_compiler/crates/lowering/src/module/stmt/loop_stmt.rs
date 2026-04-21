use ir::IrItem;
use swc_ecma_ast::{self as ast};

use crate::statements::stmt_to_ir;

pub(crate) fn lower(stmt: &ast::Stmt, items: &mut Vec<IrItem>) {
    let ir_stmt = stmt_to_ir(stmt);
    items.push(IrItem::Block(vec![ir_stmt]));
}
