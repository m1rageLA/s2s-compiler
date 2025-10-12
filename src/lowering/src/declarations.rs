use ir::{IrFunction, IrParam, IrType, IrVariable};
use swc_ecma_ast::{self as ast};

use crate::expressions::expr_to_ir;
use crate::statements::block_to_ir;
use crate::types::ts_type_ann_to_ir;

pub(crate) fn var_decl_to_ir(decl: &ast::VarDeclarator) -> Option<IrVariable> {
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

    Some(IrVariable { name, ty, value })
}

pub(crate) fn fn_decl_to_ir(fn_decl: &ast::FnDecl) -> Option<IrFunction> {
    let name = fn_decl.ident.sym.to_string();

    let mut params: Vec<IrParam> = Vec::new();
    for p in &fn_decl.function.params {
        match &p.pat {
            ast::Pat::Ident(ast::BindingIdent { id, type_ann }) => {
                let param_name = id.sym.to_string();
                let param_ty = type_ann
                    .as_ref()
                    .map(|ann| ts_type_ann_to_ir(ann))
                    .unwrap_or(IrType::Any);
                params.push(IrParam {
                    name: param_name,
                    ty: param_ty,
                });
            }
            _ => return None,
        }
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

    Some(IrFunction {
        name,
        params,
        ret: ret_ty,
        body,
    })
}
