use ir::{IrItem, IrModule};
use swc_ecma_ast::{self as ast};

mod stmt;
mod unsupported;

pub fn ast_to_ir(module: &ast::Module) -> IrModule {
    let mut items: Vec<IrItem> = Vec::new();

    for statement in &module.body {
        match statement {
            ast::ModuleItem::Stmt(stmt) => stmt::lower(stmt, &mut items),
            _ => unsupported::handle(),
        }
    }

    IrModule { items }
}
