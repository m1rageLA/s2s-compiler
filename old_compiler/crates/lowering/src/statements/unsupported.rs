use ir::IrStmt;
use swc_ecma_ast::{self as ast};

pub(crate) fn lower(_stmt: &ast::Stmt) -> IrStmt {
    IrStmt::Unsupported("stmt".into())
}
