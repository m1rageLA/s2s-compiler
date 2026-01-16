use ir::{IrStmt, IrSwitchCase};
use swc_ecma_ast as ast;

use crate::expressions::expr_to_ir;

pub(crate) fn lower(stmt: &ast::SwitchStmt) -> IrStmt {
    let discriminant = expr_to_ir(&stmt.discriminant);
    let cases = stmt.cases.iter().map(lower_case).collect();

    IrStmt::Switch {
        discriminant,
        cases,
    }
}

fn lower_case(case: &ast::SwitchCase) -> IrSwitchCase {
    let test = case.test.as_ref().map(|expr| expr_to_ir(expr));
    let consequent = case.cons.iter().map(super::stmt_to_ir).collect();
    IrSwitchCase { test, consequent }
}
