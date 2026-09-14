use logger::unsupported;
use swc_ecma_ast::{Decl, Module, ModuleItem, Stmt};

use derive_more::From;

mod transformers;

#[derive(From)]
enum AllNodes {
    ModuleItem(ModuleItem),
    Stmt(Stmt),
    Decl(Decl),
}

pub fn transform<T: Into<AllNodes>>(node: T) {
    match node.into() {
        AllNodes::ModuleItem(node) => handle_module_item(&node),
        AllNodes::Stmt(node) => handle_stmt(&node),
        AllNodes::Decl(node) => handle_decl(&node),
    }
}

fn handle_module_item(node: &ModuleItem) {
    match node {
        ModuleItem::Stmt(stmt) => handle_stmt(&stmt),

        _ => unsupported!(node),
    }
}

// -------------------------------
// LEVEL 2
// ------------------------------
fn handle_stmt(node: &Stmt) {
    match node {
        Stmt::Decl(decl) => handle_decl(decl),

        _ => unsupported!(node),
    }
}
// -------------------------------
// LEVEL 3
// ------------------------------
fn handle_decl(decl: &Decl) {
    match decl {
        Decl::Var(var) => {
            transformers::var_decl::transform_var_decl(var);
        }

        _ => unsupported!(decl),
    }
}
