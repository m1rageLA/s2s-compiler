use ir::IrType;
use swc_ecma_ast::{self as ast};

pub(crate) fn ts_type_ann_to_ir(ann: &ast::TsTypeAnn) -> IrType {
    match &*ann.type_ann {
        ast::TsType::TsKeywordType(keyword) => match keyword.kind {
            ast::TsKeywordTypeKind::TsStringKeyword => IrType::Str,
            ast::TsKeywordTypeKind::TsNumberKeyword => IrType::Number,
            ast::TsKeywordTypeKind::TsBooleanKeyword => IrType::Bool,
            _ => IrType::Any,
        },
        _ => IrType::Any,
    }
}
