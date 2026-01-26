use ir::{IrArrayKind, IrType, IrTypeAliasDef};
use swc_ecma_ast::{self as ast};
use crate::context;

mod keyword;
mod unknown;

pub(crate) fn ts_type_ann_to_ir(ann: &ast::TsTypeAnn) -> IrType {
    ts_type_to_ir(&ann.type_ann)
}

pub(crate) fn ts_type_to_ir(ty: &ast::TsType) -> IrType {
    match ty {
        ast::TsType::TsKeywordType(keyword) => keyword::from_keyword(keyword),
        ast::TsType::TsArrayType(array) => {
            let element_ty = ts_type_to_ir(&array.elem_type);
            IrType::Array(array_kind_from_type(element_ty))
        }
        ast::TsType::TsTypeRef(type_ref) => match &type_ref.type_name {
            ast::TsEntityName::Ident(ident)
                if ident.sym == *"Array" || ident.sym == *"ReadonlyArray" =>
            {
                let element_ty = type_ref
                    .type_params
                    .as_ref()
                    .and_then(|params| params.params.first())
                    .map(|ty| ts_type_to_ir(ty))
                    .unwrap_or(IrType::Any);

                IrType::Array(array_kind_from_type(element_ty))
            }
            ast::TsEntityName::Ident(ident) => {
                if let Some(alias) = context::lookup_type_alias(&ident.sym.to_string()) {
                    match alias.def {
                        IrTypeAliasDef::Object(_) => IrType::Object(alias.id),
                        IrTypeAliasDef::Alias(inner) => inner,
                    }
                } else {
                    unknown::any()
                }
            }
            _ => unknown::any(),
        },
        ast::TsType::TsParenthesizedType(inner) => ts_type_to_ir(&inner.type_ann),
        _ => unknown::any(),
    }
}

fn array_kind_from_type(element_ty: IrType) -> IrArrayKind {
    match element_ty {
        IrType::Number | IrType::UInt => IrArrayKind::Number,
        IrType::Str => IrArrayKind::Str,
        IrType::Bool => IrArrayKind::Bool,
        IrType::Value => IrArrayKind::Value,
        IrType::Any => IrArrayKind::Any,
        IrType::Object(id) => IrArrayKind::Object(id),
        _ => IrArrayKind::Unknown,
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

        let string_array = infer_type("const mixed: Array<string> = [];");
        assert_eq!(string_array, IrType::Array(IrArrayKind::Str));
    }

    #[test]
    fn falls_back_to_any_for_unknown_types() {
        let custom_ty = infer_type("const values: CustomType = [];");
        assert_eq!(custom_ty, IrType::Any);
    }
}
