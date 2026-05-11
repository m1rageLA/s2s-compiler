use ir::{IrCatchClause, IrStmt};
use swc_ecma_ast as ast;

pub(crate) fn lower(stmt: &ast::TryStmt) -> IrStmt {
    let try_block = super::block::block_to_ir(&stmt.block);

    let catch = stmt.handler.as_ref().map(|handler| {
        let param = handler.param.as_ref().and_then(|pat| match pat {
            ast::Pat::Ident(ident) => Some(ident.id.sym.to_string()),
            _ => None,
        });

        IrCatchClause {
            param,
            body: super::block::block_to_ir(&handler.body),
        }
    });

    let finally = stmt
        .finalizer
        .as_ref()
        .map(|block| super::block::block_to_ir(block));

    IrStmt::Try {
        try_block,
        catch,
        finally,
    }
}
