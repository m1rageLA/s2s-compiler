use ir::{IrType, IrVariable};
use swc_ecma_ast::{self as ast};

use crate::expressions::expr_to_ir;
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
