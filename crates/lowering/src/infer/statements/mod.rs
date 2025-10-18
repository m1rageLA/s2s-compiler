use ir::{IrStmt, IrType};

mod block;
mod do_while_loop;
mod for_loop;
mod if_branch;
mod noop;
mod return_with_value;
mod return_without_value;
mod while_loop;

pub(crate) fn collect_return_types(
    stmts: &[IrStmt],
    inferred: &mut Option<IrType>,
    saw_return: &mut bool,
) -> bool {
    for stmt in stmts {
        if !match stmt {
            IrStmt::Return(Some(expr)) => return_with_value::handle(expr, inferred, saw_return),
            IrStmt::Return(None) => return_without_value::handle(inferred, saw_return),
            IrStmt::Block(inner) => block::handle(inner, inferred, saw_return),
            IrStmt::If {
                then_branch,
                else_branch,
                ..
            } => if_branch::handle(then_branch, else_branch.as_deref(), inferred, saw_return),
            IrStmt::While(_, body) => while_loop::handle(body, inferred, saw_return),
            IrStmt::DoWhile(body, _) => do_while_loop::handle(body, inferred, saw_return),
            IrStmt::For { body, .. } => for_loop::handle(body, inferred, saw_return),
            _ => noop::handle(),
        } {
            return false;
        }
    }
    true
}
