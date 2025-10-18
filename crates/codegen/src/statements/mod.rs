use ir::IrStmt;
use proc_macro2::TokenStream;

use crate::Codegen;

mod block;
mod do_while;
mod expression;
mod for_loop;
mod if_stmt;
mod let_stmt;
mod return_stmt;
mod unsupported;
mod var_decl;
mod while_loop;

use block::block_tokens;
use do_while::do_while_tokens;
use expression::expression_stmt_tokens;
use for_loop::for_loop_tokens;
use if_stmt::if_tokens;
use let_stmt::let_tokens;
use return_stmt::return_tokens;
use unsupported::unsupported_stmt;
use var_decl::var_decl_tokens;
use while_loop::while_tokens;

impl Codegen for IrStmt {
    type Output = TokenStream;

    fn codegen(&self) -> TokenStream {
        match self {
            IrStmt::Leteral(variable) => let_tokens(variable),
            IrStmt::Expression(expr) => expression_stmt_tokens(expr),
            IrStmt::Return(expr) => return_tokens(expr.as_ref()),
            IrStmt::Block(stmts) => block_tokens(stmts),
            IrStmt::If {
                condition,
                then_branch,
                else_branch,
            } => if_tokens(condition, then_branch, else_branch.as_deref()),
            IrStmt::While(condition, body) => while_tokens(condition, body),
            IrStmt::DoWhile(body, condition) => do_while_tokens(body, condition),
            IrStmt::For {
                init,
                condition,
                update,
                body,
            } => for_loop_tokens(init.as_ref(), condition.as_ref(), update.as_ref(), body),
            IrStmt::VarDecl(vars) => var_decl_tokens(vars),
            IrStmt::Unsupported(reason) => unsupported_stmt(reason),
        }
    }
}

fn collect_stmt_tokens(stmts: &[IrStmt]) -> Vec<TokenStream> {
    stmts.iter().map(|stmt| stmt.codegen()).collect()
}
