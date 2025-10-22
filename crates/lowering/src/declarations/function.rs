use ir::{IrFunction, IrType};
use swc_ecma_ast::{self as ast};

use crate::context;
use crate::infer::infer_function_return_type;
use crate::params::params_to_ir;
use crate::statements::block_to_ir;
use crate::types::ts_type_ann_to_ir;

pub(crate) fn fn_decl_to_ir(fn_decl: &ast::FnDecl) -> Option<IrFunction> {
    let name = fn_decl.ident.sym.to_string();

    let params = params_to_ir(&fn_decl.function.params);

    context::push_scope();
    for param in &params {
        context::define(&param.name, param.ty);
    }

    let ret_ty = fn_decl
        .function
        .return_type
        .as_ref()
        .map(|ann| ts_type_ann_to_ir(ann))
        .unwrap_or(IrType::Any);

    let body = fn_decl
        .function
        .body
        .as_ref()
        .map(|block| block_to_ir(block))
        .unwrap_or_default();

    context::pop_scope();

    let mut ir_function = IrFunction {
        name,
        params,
        ret: ret_ty,
        body,
    };

    if matches!(ir_function.ret, IrType::Any) {
        if let Some(inferred) = infer_function_return_type(&ir_function.body) {
            ir_function.ret = inferred;
        }
    }

    Some(ir_function)
}
