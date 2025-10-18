use ir::IrStmt;
use swc_ecma_ast::{self as ast};

mod block;
mod block_like;
mod do_while_stmt;
mod expr_stmt;
mod for_stmt;
mod if_stmt;
mod return_stmt;
mod unsupported;
mod var_decl_stmt;
mod while_stmt;

pub(crate) use block::block_to_ir;
pub(crate) use block_like::stmt_block_like_to_ir;

pub(crate) fn stmt_to_ir(stmt: &ast::Stmt) -> IrStmt {
    match stmt {
        ast::Stmt::Expr(expr_stmt) => expr_stmt::lower(expr_stmt),
        ast::Stmt::Return(ret_stmt) => return_stmt::lower(ret_stmt),
        ast::Stmt::Decl(ast::Decl::Var(var_decl)) => var_decl_stmt::lower(var_decl),
        ast::Stmt::Block(block) => block::from_block(block),
        ast::Stmt::If(if_stmt) => if_stmt::lower(if_stmt),
        ast::Stmt::While(while_stmt) => while_stmt::lower(while_stmt),
        ast::Stmt::DoWhile(do_while_stmt) => do_while_stmt::lower(do_while_stmt),
        ast::Stmt::For(for_stmt) => for_stmt::lower(for_stmt),
        _ => unsupported::lower(stmt),
    }
}
