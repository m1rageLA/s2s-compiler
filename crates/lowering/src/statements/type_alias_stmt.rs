use ir::{IrObjectField, IrStmt, IrType, IrTypeAliasDef};
use swc_ecma_ast as ast;

use crate::context;
use crate::types::{ts_type_ann_to_ir, ts_type_to_ir};

pub(crate) fn lower(type_alias: &ast::TsTypeAliasDecl) -> IrStmt {
    let name = type_alias.id.sym.to_string();
    let def = type_alias_def_from_ts_type(&type_alias.type_ann);
    let alias = context::define_type_alias(&name, def);
    IrStmt::TypeAlias(alias)
}

pub(crate) fn type_alias_def_from_ts_type(ty: &ast::TsType) -> IrTypeAliasDef {
    match ty {
        ast::TsType::TsTypeLit(type_lit) => {
            let fields = type_lit
                .members
                .iter()
                .filter_map(|member| match member {
                    ast::TsTypeElement::TsPropertySignature(prop) => {
                        let name = match prop.key.as_ref() {
                            ast::Expr::Ident(ident) => ident.sym.to_string(),
                            ast::Expr::Lit(ast::Lit::Str(str)) => str.value.to_string(),
                            ast::Expr::Lit(ast::Lit::Num(num)) => num.value.to_string(),
                            ast::Expr::Lit(ast::Lit::Bool(b)) => b.value.to_string(),
                            ast::Expr::Lit(ast::Lit::BigInt(b)) => b.value.to_string(),
                            _ => "unsupported".to_string(),
                        };
                        let ty = prop
                            .type_ann
                            .as_ref()
                            .map(|ann| ts_type_ann_to_ir(ann))
                            .unwrap_or(IrType::Any);
                        Some(IrObjectField { name, ty })
                    }
                    _ => None,
                })
                .collect::<Vec<_>>();
            IrTypeAliasDef::Object(fields)
        }
        _ => IrTypeAliasDef::Alias(ts_type_to_ir(ty)),
    }
}
