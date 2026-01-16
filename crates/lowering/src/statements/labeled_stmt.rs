use ir::IrStmt;
use swc_ecma_ast as ast;

pub(crate) fn lower(stmt: &ast::LabeledStmt) -> IrStmt {
    IrStmt::Labeled {
        label: stmt.label.sym.to_string(),
        body: Box::new(super::stmt_to_ir(&stmt.body)),
    }
}
