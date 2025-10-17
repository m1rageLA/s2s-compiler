use ir::{IrFunction, IrType, IrVariable};
use swc_ecma_ast::{self as ast};

use crate::expressions::expr_to_ir;
use crate::infer::infer_function_return_type;
use crate::params::params_to_ir;
use crate::statements::block_to_ir;
use crate::types::ts_type_ann_to_ir;

pub(crate) fn var_decl_to_ir(
    decl: &ast::VarDeclarator,
    kind: ast::VarDeclKind,
) -> Option<IrVariable> {
    let name = match &decl.name {
        ast::Pat::Ident(ident) => ident.id.sym.to_string(),
        _ => return None,
    };

    let ty = match &decl.name {
        ast::Pat::Ident(ident) => ident
            .type_ann
            .as_ref()
            .map(|ann| ts_type_ann_to_ir(ann))
            .unwrap_or(IrType::Any),
        _ => IrType::Any,
    };

    let value = decl.init.as_ref().map(|expr| expr_to_ir(expr));
    let mutable = !matches!(kind, ast::VarDeclKind::Const);

    Some(IrVariable {
        name,
        mutable,
        ty,
        value,
    })
}

pub(crate) fn fn_decl_to_ir(fn_decl: &ast::FnDecl) -> Option<IrFunction> {
    let name = fn_decl.ident.sym.to_string();

    let params = params_to_ir(&fn_decl.function.params);

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
