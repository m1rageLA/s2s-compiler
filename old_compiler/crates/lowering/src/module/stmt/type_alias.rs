use ir::{IrItem, IrStmt};
use swc_ecma_ast as ast;

use crate::statements::type_alias_stmt;

pub(crate) fn lower(type_alias: &ast::TsTypeAliasDecl, items: &mut Vec<IrItem>) {
    let stmt = type_alias_stmt::lower(type_alias);
    if let IrStmt::TypeAlias(alias) = stmt {
        items.push(IrItem::TypeAlias(alias));
    }
}
