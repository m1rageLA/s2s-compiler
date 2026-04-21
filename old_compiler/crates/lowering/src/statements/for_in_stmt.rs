use ir::{IrExpression, IrForInLeft, IrStmt, IrType};
use swc_ecma_ast::{self as ast};

use crate::declarations::var_decl_to_ir;
use crate::expressions::expr_to_ir;

pub(crate) fn lower(stmt: &ast::ForInStmt) -> IrStmt {
    let left = match &stmt.left {
        ast::ForHead::VarDecl(var_decl) => {
            let kind = var_decl.kind;
            var_decl
                .decls
                .iter()
                .filter_map(|decl| var_decl_to_ir(decl, kind))
                .next()
                .map(|mut var| {
                    if matches!(var.ty, IrType::Any | IrType::Value) {
                        var.ty = IrType::Str;
                    }
                    IrForInLeft::Var(var)
                })
                .unwrap_or_else(|| {
                    IrForInLeft::Pattern(IrExpression::Identifier(
                        "unsupported_for_in_lhs".into(),
                    ))
                })
        }
        ast::ForHead::Pat(pat) => match pat.as_ref() {
            ast::Pat::Ident(ident) => IrForInLeft::Identifier(ident.id.sym.to_string()),
            ast::Pat::Expr(expr) => IrForInLeft::Pattern(expr_to_ir(expr)),
            _ => IrForInLeft::Pattern(IrExpression::Identifier(
                "unsupported_for_in_pattern".into(),
            )),
        },
        _ => IrForInLeft::Pattern(IrExpression::Identifier(
            "unsupported_for_in_lhs".into(),
        )),
    };

    let right = expr_to_ir(&stmt.right);
    let body = super::stmt_block_like_to_ir(&stmt.body);

    IrStmt::ForIn { left, right, body }
}
