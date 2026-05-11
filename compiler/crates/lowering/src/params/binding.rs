use ir::{IrParam, IrType};
use swc_ecma_ast::{self as ast};

use crate::types::ts_type_ann_to_ir;

pub(crate) fn from_binding(binding: &ast::BindingIdent) -> IrParam {
    let name = binding.id.sym.to_string();
    let ty = binding
        .type_ann
        .as_ref()
        .map(|ann| ts_type_ann_to_ir(ann))
        .unwrap_or(IrType::Any);

    IrParam { name, ty }
}
