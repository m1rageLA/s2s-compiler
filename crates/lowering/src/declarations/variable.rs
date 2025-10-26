use ir::{IrExpression, IrType, IrVariable, RuntimeNamespace};
use swc_ecma_ast::{self as ast};

use crate::context;
use crate::expressions::{coerce_to_value, expr_to_ir};
use crate::infer;
use crate::types::ts_type_ann_to_ir;

pub(crate) fn var_decl_to_ir(
    decl: &ast::VarDeclarator,
    kind: ast::VarDeclKind,
) -> Option<IrVariable> {
    let name = match &decl.name {
        ast::Pat::Ident(ident) => ident.id.sym.to_string(),
        _ => return None,
    };

    let mut ty = match &decl.name {
        ast::Pat::Ident(ident) => ident
            .type_ann
            .as_ref()
            .map(|ann| ts_type_ann_to_ir(ann))
            .unwrap_or(IrType::Any),
        _ => IrType::Any,
    };

    let mut value = decl.init.as_ref().map(|expr| expr_to_ir(expr));

    if matches!(ty, IrType::Any) {
        if let Some(expr) = value.as_ref() {
            if let Some(inferred) = infer::infer_expression_type(expr) {
                ty = inferred;
            }
        }
    }

    // If the declared/final variable type is Value or Str, coerce the initializer
    // to Value so runtime semantics live in IR (and codegen emits Value usage).
    if matches!(ty, IrType::Value | IrType::Str) {
        if let Some(init_expr) = value.take() {
            let preserve_literal = matches!(ty, IrType::Value | IrType::Str)
                && matches!(
                    init_expr,
                    IrExpression::Literal(_)
                        | IrExpression::Template(_)
                        | IrExpression::RuntimeCall(RuntimeNamespace::Value(_))
                );

            if preserve_literal {
                value = Some(init_expr);
            } else if matches!(
                init_expr,
                IrExpression::RuntimeCall(RuntimeNamespace::Value(_))
                    | IrExpression::Function(_)
                    | IrExpression::Arrow { .. }
            ) {
                value = Some(init_expr);
            } else {
                value = Some(coerce_to_value(init_expr));
            }
        }
    }
    let mutable = !matches!(kind, ast::VarDeclKind::Const);

    context::define(&name, ty);

    Some(IrVariable {
        name,
        mutable,
        ty,
        value,
    })
}
