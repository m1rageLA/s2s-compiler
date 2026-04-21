use ir::IrStmt;
use swc_ecma_ast::{self as ast};

use crate::declarations::var_decl_to_ir;

pub(crate) fn lower(var_decl: &ast::VarDecl) -> IrStmt {
    let kind = var_decl.kind;
    let vars = var_decl
        .decls
        .iter()
        .filter_map(|decl| var_decl_to_ir(decl, kind))
        .collect::<Vec<_>>();
    IrStmt::VarDecl(vars)
}
