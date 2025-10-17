use codegen::Codegen;
use ir::{IrExpression, IrFunction, IrItem, IrModule, IrStmt, IrType};

#[test]
fn module_codegen_emits_main_and_function_items() {
    let helper_function = IrItem::Function(IrFunction {
        name: "helper".into(),
        params: Vec::new(),
        ret: IrType::Unit,
        body: vec![IrStmt::Return(None)],
    });

    let call_expression = IrItem::Expression(IrExpression::Call {
        callee: Box::new(IrExpression::Identifier("helper".into())),
        args: Vec::new(),
    });

    let ir_module = IrModule {
        items: vec![helper_function, call_expression],
    };

    let tokens = ir_module.codegen();

    let file: syn::File = syn::parse2(tokens).expect("generated Rust should parse");

    let helper_fn = file.items.iter().find_map(|item| match item {
        syn::Item::Fn(func) if func.sig.ident == "helper" => Some(func),
        _ => None,
    })
    .expect("expected helper function to be generated");

    let main_fn = file.items.iter().find_map(|item| match item {
        syn::Item::Fn(func) if func.sig.ident == "main" => Some(func),
        _ => None,
    })
    .expect("expected main function to be generated");

    let helper_stmt = helper_fn
        .block
        .stmts
        .first()
        .expect("helper function should contain a statement");

    match helper_stmt {
        syn::Stmt::Expr(expr, _) => match expr {
            syn::Expr::Return(ret) => {
                assert!(ret.expr.is_none(), "helper should return without a value");
            }
            _ => panic!("expected return expression inside helper"),
        },
        _ => panic!("expected return statement inside helper"),
    }

    let main_stmt = main_fn
        .block
        .stmts
        .first()
        .expect("main function should call helper");

    match main_stmt {
        syn::Stmt::Expr(expr, _) => match expr {
            syn::Expr::Call(call) => {
                let called = extract_ident(call.func.as_ref()).expect("call should target an identifier");
                assert_eq!(called, "helper");
                assert!(call.args.is_empty(), "helper call should not pass arguments");
            }
            _ => panic!("expected helper call expression inside main"),
        },
        _ => panic!("expected helper call inside main"),
    }
}

fn extract_ident(expr: &syn::Expr) -> Option<&syn::Ident> {
    match expr {
        syn::Expr::Path(path) => path.path.get_ident(),
        syn::Expr::Paren(paren) => extract_ident(&paren.expr),
        syn::Expr::Group(group) => extract_ident(&group.expr),
        _ => None,
    }
}
