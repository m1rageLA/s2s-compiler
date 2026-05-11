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
            IrStmt::ForIn { body, .. } => for_loop::handle(body, inferred, saw_return),
            IrStmt::Labeled { body, .. } => {
                collect_return_types(std::slice::from_ref(body), inferred, saw_return)
            }
            IrStmt::Switch { cases, .. } => cases
                .iter()
                .all(|case| collect_return_types(&case.consequent, inferred, saw_return)),
            IrStmt::Try {
                try_block,
                catch,
                finally,
            } => {
                let mut ok = collect_return_types(try_block, inferred, saw_return);
                if let Some(handler) = catch {
                    ok &= collect_return_types(&handler.body, inferred, saw_return);
                }
                if let Some(finally) = finally {
                    ok &= collect_return_types(finally, inferred, saw_return);
                }
                ok
            }
            IrStmt::Empty | IrStmt::Break(_) | IrStmt::Continue(_) | IrStmt::TypeAlias(_) => {
                noop::handle()
            }
            _ => noop::handle(),
        } {
            return false;
        }
    }
    true
}
