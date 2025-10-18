use ir::IrItem;
use swc_ecma_ast::{self as ast};

mod block_item;
mod expression_item;
mod function_decl;
mod loop_stmt;
mod unsupported_stmt;
mod var_decl;

pub(crate) fn lower(stmt: &ast::Stmt, items: &mut Vec<IrItem>) {
    match stmt {
        ast::Stmt::Decl(ast::Decl::Var(var_decl)) => var_decl::lower(var_decl, items),
        ast::Stmt::Decl(ast::Decl::Fn(fn_decl)) => function_decl::lower(fn_decl, items),
        ast::Stmt::Expr(expr_stmt) => expression_item::lower(expr_stmt, items),
        ast::Stmt::Block(block) => block_item::lower(block, items),
        ast::Stmt::While(_) | ast::Stmt::DoWhile(_) | ast::Stmt::For(_) => {
            loop_stmt::lower(stmt, items)
        }
        _ => unsupported_stmt::handle(),
    }
}
