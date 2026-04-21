use ir::IrType;
use swc_ecma_ast::{self as ast};

pub(crate) fn from_keyword(keyword: &ast::TsKeywordType) -> IrType {
    match keyword.kind {
        ast::TsKeywordTypeKind::TsStringKeyword => IrType::Str,
        ast::TsKeywordTypeKind::TsNumberKeyword => IrType::Number,
        ast::TsKeywordTypeKind::TsBooleanKeyword => IrType::Bool,
        ast::TsKeywordTypeKind::TsVoidKeyword | ast::TsKeywordTypeKind::TsUndefinedKeyword => {
            IrType::Unit
        }
        _ => IrType::Any,
    }
}
