// Re-export shared types and aliases for submodules
pub(crate) use ir::{
    IrBinOp, IrExpression, IrLiteral, IrTemplatePart, RuntimeNamespace,
};
pub(crate) use swc_ecma_ast as ast;

mod binary;
mod unary;
mod call;
mod arrow;
mod literal;
mod template;
mod array;
// helpers removed; we colocate simple helpers in existing modules

pub use binary::*;
pub use unary::*;
pub use call::*;
pub use arrow::*;
pub use literal::*;
pub use template::*;
pub use array::*;

pub(crate) fn expr_to_ir(expr: &ast::Expr) -> IrExpression {
    match expr {
        ast::Expr::Lit(ast::Lit::Num(n)) => number_lit_to_ir(n),
        ast::Expr::Lit(ast::Lit::Str(s)) => string_lit_to_ir(s),
        ast::Expr::Lit(ast::Lit::Bool(b)) => bool_lit_to_ir(b),
        ast::Expr::Ident(i) => ident_to_ir(i),
        ast::Expr::Paren(p) => paren_to_ir(p),
        ast::Expr::Bin(b) => binary_expr_to_ir(b),
        ast::Expr::Unary(u) => unary_expr_to_ir(u),
        ast::Expr::Call(call) => call_to_ir(call),
        ast::Expr::Array(a) => array_expr_to_ir(a),
        ast::Expr::Tpl(tpl) => template_expr_to_ir(tpl),

        _ => IrExpression::Identifier("unsupported".to_string()),
    }
}
