// Re-export shared types and aliases for submodules
pub(crate) use ir::{IrArrowBody, IrAssignOp, IrBinOp, IrExpression, IrLiteral, IrTemplatePart};
pub(crate) use swc_ecma_ast as ast;

mod array;
mod arrow;
mod assignment;
mod binary;
mod call;
mod conditional;
mod function;
mod literal;
mod member;
mod object;
mod template;
mod unary;
mod update;

#[allow(unused_imports)]
pub use array::*;
pub use arrow::*;
#[allow(unused_imports)]
pub use assignment::*;
#[allow(unused_imports)]
pub use binary::*;
pub use call::*;
#[allow(unused_imports)]
pub use conditional::*;
pub use function::*;
#[allow(unused_imports)]
pub use literal::*;
#[allow(unused_imports)]
pub use member::*;
#[allow(unused_imports)]
pub use object::*;
#[allow(unused_imports)]
pub use template::*;
#[allow(unused_imports)]
pub use unary::*;
#[allow(unused_imports)]
pub use update::*;

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
        ast::Expr::Object(obj) => object_expr_to_ir(obj),
        ast::Expr::Tpl(tpl) => template_expr_to_ir(tpl),
        ast::Expr::Arrow(arw) => arrow_expr_to_ir(arw),
        ast::Expr::Cond(cond) => cond_expr_to_ir(cond),
        ast::Expr::Update(u) => update_expr_to_ir(u),
        ast::Expr::Fn(fn_expr) => function_expr_to_ir(fn_expr),
        ast::Expr::Member(member) => {
            let member_expr = lower_member_expr(member);
            runtime_value_for_member(&member_expr).unwrap_or(member_expr)
        }
        ast::Expr::Assign(assign) => assignment_expr_to_ir(assign),
        _ => IrExpression::Identifier("unsupported".to_string()),
    }
}
