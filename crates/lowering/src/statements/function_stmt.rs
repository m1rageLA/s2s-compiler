use ir::{IrExpression, IrStmt, IrType, IrVariable};
use swc_ecma_ast::{self as ast};

use crate::context;
use crate::expressions::function_decl_to_expr;

pub(crate) fn lower(fn_decl: &ast::FnDecl) -> IrStmt {
    let name = fn_decl.ident.sym.to_string();
    let expr = function_decl_to_expr(fn_decl);

    match expr {
        IrExpression::Function(func) => {
            context::define_function_return(&name, func.ret);
            context::define(&name, IrType::Any);
            IrStmt::VarDecl(vec![IrVariable {
                name: name.clone(),
                mutable: false,
                ty: IrType::Any,
                value: Some(IrExpression::Function(func)),
            }])
        }
        IrExpression::Identifier(reason) if reason.ends_with("_not_supported") => {
            IrStmt::Unsupported(reason)
        }
        other => {
            context::define(&name, IrType::Any);
            IrStmt::VarDecl(vec![IrVariable {
                name: name.clone(),
                mutable: false,
                ty: IrType::Any,
                value: Some(other),
            }])
        }
    }
}
