use ir::IrParam;
use swc_ecma_ast::{self as ast};

use super::{binding, unsupported};

pub(crate) fn from_pat(pat: &ast::Pat) -> IrParam {
    match pat {
        ast::Pat::Ident(binding_ident) => binding::from_binding(binding_ident),
        ast::Pat::Assign(assign) => from_pat(&assign.left),
        ast::Pat::Rest(rest) => from_pat(&rest.arg),
        _ => unsupported::make(),
    }
}
