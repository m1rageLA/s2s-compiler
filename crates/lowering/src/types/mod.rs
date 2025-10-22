use ir::{IrArrayKind, IrType};
use swc_ecma_ast::{self as ast};

mod keyword;
mod unknown;

pub(crate) fn ts_type_ann_to_ir(ann: &ast::TsTypeAnn) -> IrType {
    ts_type_to_ir(&ann.type_ann)
}

fn ts_type_to_ir(ty: &ast::TsType) -> IrType {
    match ty {
        ast::TsType::TsKeywordType(keyword) => keyword::from_keyword(keyword),
        ast::TsType::TsArrayType(array) => {
            let element_ty = ts_type_to_ir(&array.elem_type);
            let kind = match element_ty {
                IrType::Number => IrArrayKind::Number,
                IrType::Str => IrArrayKind::Str,
                IrType::Bool => IrArrayKind::Bool,
                IrType::Value => IrArrayKind::Value,
                IrType::Any => IrArrayKind::Any,
                _ => IrArrayKind::Unknown,
            };
            IrType::Array(kind)
        }
        ast::TsType::TsParenthesizedType(inner) => ts_type_to_ir(&inner.type_ann),
        _ => unknown::any(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use swc_ecma_ast::{ModuleItem, VarDeclKind};

    fn infer_type(source: &str) -> IrType {
        let module = parser::ast(source);
        let var_decl = match module.body.first().expect("expected variable declaration") {
            ModuleItem::Stmt(ast::Stmt::Decl(ast::Decl::Var(var_decl))) => var_decl,
            other => panic!("expected variable declaration, got {other:?}"),
        };

        assert_eq!(var_decl.kind, VarDeclKind::Const);
        let decl = var_decl.decls.first().expect("expected declarator");
        let ident = match &decl.name {
            ast::Pat::Ident(ident) => ident,
            other => panic!("expected identifier pattern, got {other:?}"),
        };

        let ann = ident
            .type_ann
            .as_ref()
            .expect("expected type annotation on identifier");

        ts_type_ann_to_ir(ann)
    }

    #[test]
    fn maps_keyword_types() {
        let string_ty = infer_type("const label: string = 'hi';");
        assert_eq!(string_ty, IrType::Str);

        let number_ty = infer_type("const total: number = 1;");
        assert_eq!(number_ty, IrType::Number);

        let bool_ty = infer_type("const flag: boolean = true;");
        assert_eq!(bool_ty, IrType::Bool);
    }

    #[test]
    fn maps_array_types() {
        let number_array = infer_type("const values: number[] = [];");
        assert_eq!(number_array, IrType::Array(IrArrayKind::Number));

        let unknown_array = infer_type("const mixed: Array<string> = [];");
        assert_eq!(unknown_array, IrType::Any);
    }

    #[test]
    fn falls_back_to_any_for_unknown_types() {
        let custom_ty = infer_type("const values: Array<number> = [];");
        assert_eq!(custom_ty, IrType::Any);
    }
}
