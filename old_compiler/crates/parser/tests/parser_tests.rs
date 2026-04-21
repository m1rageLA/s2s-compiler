use parser::ast;
use swc_ecma_ast::{Decl, Expr, ModuleItem, Pat, Stmt, TsKeywordTypeKind, TsType};

#[test]
fn parses_variable_declaration_into_ast_module() {
    let module = ast("let answer = 42;");

    assert_eq!(module.body.len(), 1, "expected exactly one top-level item");

    match &module.body[0] {
        ModuleItem::Stmt(Stmt::Decl(Decl::Var(var_decl))) => {
            assert_eq!(var_decl.decls.len(), 1, "expected a single declarator");
            match &var_decl.decls[0].name {
                Pat::Ident(ident) => assert_eq!(ident.id.sym, *"answer"),
                other => panic!("expected identifier pattern, got {other:?}"),
            }
        }
        other => panic!("expected variable declaration, got {other:?}"),
    }
}

#[test]
fn keeps_ts_type_annotations() {
    let module = ast("let answer: number = 42;");

    let ModuleItem::Stmt(Stmt::Decl(Decl::Var(var_decl))) = &module.body[0] else {
        panic!("expected var declaration");
    };

    let Pat::Ident(binding) = &var_decl.decls[0].name else {
        panic!("expected identifier binding");
    };

    let Some(type_ann) = &binding.type_ann else {
        panic!("expected type annotation to stay on binding");
    };

    match type_ann.type_ann.as_ref() {
        TsType::TsKeywordType(keyword) => {
            assert!(
                matches!(keyword.kind, TsKeywordTypeKind::TsNumberKeyword),
                "expected number keyword, got {:?}",
                keyword.kind
            );
        }
        other => panic!("expected keyword type, got {other:?}"),
    }
}

#[test]
fn lowers_arrow_functions_and_restores_param_types() {
    let module = ast("const inc = (value: number) => value + 1;");

    let ModuleItem::Stmt(Stmt::Decl(Decl::Var(var_decl))) = &module.body[0] else {
        panic!("expected variable declaration");
    };

    let init = var_decl.decls[0]
        .init
        .as_ref()
        .expect("expected initializer expression");

    let Expr::Arrow(arrow) = init.as_ref() else {
        panic!("expected arrow expression, got {init:?}");
    };

    assert_eq!(arrow.params.len(), 1, "expected single param");

    let param_pat = &arrow.params[0];
    let Pat::Ident(binding) = param_pat else {
        panic!("expected ident param, got {param_pat:?}");
    };

    let Some(type_ann) = &binding.type_ann else {
        panic!("expected type annotation restored on parameter");
    };

    match type_ann.type_ann.as_ref() {
        TsType::TsKeywordType(keyword) => assert!(
            matches!(keyword.kind, TsKeywordTypeKind::TsNumberKeyword),
            "expected number keyword, got {:?}",
            keyword.kind
        ),
        other => panic!("expected keyword type, got {other:?}"),
    }
}
