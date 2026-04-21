use ir::IrStmt;
use swc_ecma_ast as ast;

pub(crate) fn lower(stmt: &ast::BreakStmt) -> IrStmt {
    IrStmt::Break(stmt.label.as_ref().map(|label| label.sym.to_string()))
}
