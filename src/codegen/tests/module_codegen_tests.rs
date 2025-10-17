use codegen::Codegen;
use ir::{
    IrExpression, IrFunction, IrFunctionExpr, IrItem, IrLiteral, IrModule, IrParam, IrStmt, IrType,
};

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

#[test]
fn function_expression_codegen_emits_closure_literal() {
    let function_expr = IrExpression::Function(Box::new(IrFunctionExpr {
        name: None,
        params: vec![IrParam {
            name: "value".into(),
            ty: IrType::Number,
        }],
        ret: IrType::Number,
        body: vec![IrStmt::Return(Some(IrExpression::Identifier("value".into())))],
    }));

    let module = IrModule {
        items: vec![IrItem::Expression(function_expr)],
    };

    let tokens = module.codegen();
    let file: syn::File = syn::parse2(tokens).expect("generated Rust should parse");

    let main_fn = file.items.iter().find_map(|item| match item {
        syn::Item::Fn(func) if func.sig.ident == "main" => Some(func),
        _ => None,
    })
    .expect("expected generated main function");

    let stmt = main_fn
        .block
        .stmts
        .first()
        .expect("main should contain closure expression");

    match stmt {
        syn::Stmt::Expr(expr, _) => match expr {
            syn::Expr::Closure(closure) => {
                assert!(closure.capture.is_some(), "closure should be move");
                assert_eq!(closure.inputs.len(), 1);
                match &closure.inputs[0] {
                    syn::Pat::Type(pat_type) => match pat_type.ty.as_ref() {
                        syn::Type::Path(path) => {
                            let ty_ident = path.path.get_ident().expect("type ident");
                            assert_eq!(ty_ident, "f64");
                        }
                        _ => panic!("expected closure arg type to be f64"),
                    },
                    _ => panic!("expected typed closure argument"),
                }
                match &closure.output {
                    syn::ReturnType::Type(_, ty) => match ty.as_ref() {
                        syn::Type::Path(path) => {
                            let ty_ident = path.path.get_ident().expect("type ident");
                            assert_eq!(ty_ident, "f64");
                        }
                        _ => panic!("expected closure return type to be f64 path"),
                    },
                    _ => panic!("expected explicit return type"),
                }
            }
            _ => panic!("expected closure expression"),
        },
        _ => panic!("expected closure statement"),
    }
}

#[test]
fn function_expression_codegen_omits_return_type_for_any() {
    let function_expr = IrExpression::Function(Box::new(IrFunctionExpr {
        name: None,
        params: vec![
            IrParam {
                name: "a".into(),
                ty: IrType::Number,
            },
            IrParam {
                name: "b".into(),
                ty: IrType::Number,
            },
        ],
        ret: IrType::Any,
        body: vec![IrStmt::Return(Some(IrExpression::Literal(
            IrLiteral::Number(42.0),
        )))],
    }));

    let module = IrModule {
        items: vec![IrItem::Expression(function_expr)],
    };

    let tokens = module.codegen();
    let file: syn::File = syn::parse2(tokens).expect("generated Rust should parse");

    let main_fn = file.items.iter().find_map(|item| match item {
        syn::Item::Fn(func) if func.sig.ident == "main" => Some(func),
        _ => None,
    })
    .expect("expected generated main function");

    let stmt = main_fn
        .block
        .stmts
        .first()
        .expect("main should contain closure expression");

    match stmt {
        syn::Stmt::Expr(expr, _) => match expr {
            syn::Expr::Closure(closure) => {
                assert!(closure.capture.is_some(), "closure should be move");
                assert_eq!(closure.inputs.len(), 2);
                assert!(matches!(closure.output, syn::ReturnType::Default));
            }
            _ => panic!("expected closure expression"),
        },
        _ => panic!("expected closure statement"),
    }
}
