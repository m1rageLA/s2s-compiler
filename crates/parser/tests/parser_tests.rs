use parser::ast;
use swc_ecma_ast::{Decl, ModuleItem, Pat, Stmt};

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
