use ir::IrStmt;
use swc_ecma_ast as ast;

pub(crate) fn lower(stmt: &ast::ContinueStmt) -> IrStmt {
    IrStmt::Continue(stmt.label.as_ref().map(|label| label.sym.to_string()))
}
