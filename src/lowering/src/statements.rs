use ir::{IrForInit, IrStmt};
use swc_ecma_ast::{self as ast};

use crate::declarations::var_decl_to_ir;
use crate::expressions::expr_to_ir;

pub(crate) fn stmt_to_ir(stmt: &ast::Stmt) -> IrStmt {
    match stmt {
        ast::Stmt::Expr(expr_stmt) => {
            let ir_expr = expr_to_ir(&expr_stmt.expr);
            IrStmt::Expression(ir_expr)
        }
        ast::Stmt::Return(ret_stmt) => {
            let value = ret_stmt.arg.as_ref().map(|expr| expr_to_ir(expr));
            IrStmt::Return(value)
        }
        ast::Stmt::Decl(ast::Decl::Var(var_decl)) => {
            let kind = var_decl.kind;
            let vars = var_decl
                .decls
                .iter()
                .filter_map(|decl| var_decl_to_ir(decl, kind))
                .collect::<Vec<_>>();
            IrStmt::VarDecl(vars)
        }
        ast::Stmt::Block(block) => IrStmt::Block(block_to_ir(block)),
        ast::Stmt::If(if_stmt) => {
            let condition = expr_to_ir(&if_stmt.test);
            let then_branch = stmt_block_like_to_ir(&if_stmt.cons);
            let else_branch = if let Some(alt) = &if_stmt.alt {
                Some(stmt_block_like_to_ir(alt))
            } else {
                None
            };
            IrStmt::If {
                condition,
                then_branch,
                else_branch,
            }
        }
        ast::Stmt::While(while_stmt) => {
            let condition = expr_to_ir(&while_stmt.test);
            let body = stmt_block_like_to_ir(&while_stmt.body);
            IrStmt::While(condition, body)
        }
        ast::Stmt::DoWhile(do_while_stmt) => {
            let body = stmt_block_like_to_ir(&do_while_stmt.body);
            let condition = expr_to_ir(&do_while_stmt.test);
            IrStmt::DoWhile(body, condition)
        }
        ast::Stmt::For(for_stmt) => for_stmt_to_ir(for_stmt),
        _ => IrStmt::Unsupported("stmt".into()),
    }
}

pub(crate) fn block_to_ir(block: &ast::BlockStmt) -> Vec<IrStmt> {
    block.stmts.iter().map(stmt_to_ir).collect()
}

fn stmt_block_like_to_ir(stmt: &ast::Stmt) -> Vec<IrStmt> {
    match stmt {
        ast::Stmt::Block(block) => block_to_ir(block),
        other => vec![stmt_to_ir(other)],
    }
}

fn for_stmt_to_ir(for_stmt: &ast::ForStmt) -> IrStmt {
    let init = match &for_stmt.init {
        Some(ast::VarDeclOrExpr::VarDecl(var_decl)) => {
            let kind = var_decl.kind;
            let vars = var_decl
                .decls
                .iter()
                .filter_map(|decl| var_decl_to_ir(decl, kind))
                .collect::<Vec<_>>();
            if vars.is_empty() {
                None
            } else {
                Some(IrForInit::VarDecl(vars))
            }
        }
        Some(ast::VarDeclOrExpr::Expr(expr)) => Some(IrForInit::Expr(expr_to_ir(expr))),
        None => None,
    };

    let condition = for_stmt.test.as_ref().map(|expr| expr_to_ir(expr));
    let update = for_stmt.update.as_ref().map(|expr| expr_to_ir(expr));
    let body = stmt_block_like_to_ir(&for_stmt.body);

    IrStmt::For {
        init,
        condition,
        update,
        body,
    }
}
