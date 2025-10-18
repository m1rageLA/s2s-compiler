use ir::IrType;
use swc_ecma_ast::{self as ast};

mod keyword;
mod unknown;

pub(crate) fn ts_type_ann_to_ir(ann: &ast::TsTypeAnn) -> IrType {
    match &*ann.type_ann {
        ast::TsType::TsKeywordType(keyword) => keyword::from_keyword(keyword),
        _ => unknown::any(),
    }
}
