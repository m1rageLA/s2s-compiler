use ir::IrItem;
use swc_ecma_ast::{self as ast};

use crate::declarations::var_decl_to_ir;

pub(crate) fn lower(var_decl: &ast::VarDecl, items: &mut Vec<IrItem>) {
    let kind = var_decl.kind;
    for decl in &var_decl.decls {
        if let Some(ir_var) = var_decl_to_ir(decl, kind) {
            items.push(IrItem::Variable(ir_var));
        }
    }
}
