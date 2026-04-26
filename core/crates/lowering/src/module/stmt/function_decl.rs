use ir::IrItem;
use swc_ecma_ast::{self as ast};

use crate::declarations::fn_decl_to_ir;

pub(crate) fn lower(fn_decl: &ast::FnDecl, items: &mut Vec<IrItem>) {
    if let Some(ir_fn) = fn_decl_to_ir(fn_decl) {
        items.push(IrItem::Function(ir_fn));
    }
}
