use ir::*;
use swc_ecma_ast::{self as ast, Number, Str};

pub fn ast_to_ir(module: &ast::Module) -> IrModule {
    let mut items: Vec<IrItem> = Vec::new();

    for statement in &module.body {
        match statement {
            ast::ModuleItem::Stmt(ast::Stmt::Decl(ast::Decl::Var(var_decl))) => {
                for decl in &var_decl.decls {
                    if let Some(ir_var) = var_decl_to_ir(decl) {
                        items.push(IrItem::Variable(ir_var));
                    }
                }
            }
            ast::ModuleItem::Stmt(ast::Stmt::Decl(ast::Decl::Fn(fn_decl))) => {
                if let Some(ir_fn) = fn_decl_to_ir(fn_decl) {
                    items.push(IrItem::Function(ir_fn));
                }
            }
            _ => (),
        }
    }
    println!("items: {:#?}", items);
    IrModule { items }
}

fn var_decl_to_ir(decl: &ast::VarDeclarator) -> Option<IrVariable> {
    let name = match &decl.name {
        ast::Pat::Ident(ident) => ident.id.sym.to_string(),
        _ => return None,
    };

    let ty = if let ast::Pat::Ident(ident) = &decl.name {
        if let Some(type_ann) = &ident.type_ann {
            match &*type_ann.type_ann {
                ast::TsType::TsKeywordType(keyword) => match keyword.kind {
                    ast::TsKeywordTypeKind::TsNumberKeyword => IrType::Int,
                    ast::TsKeywordTypeKind::TsStringKeyword => IrType::Str,
                    ast::TsKeywordTypeKind::TsBooleanKeyword => IrType::Bool,
                    _ => IrType::Any,
                },
                _ => IrType::Any,
            }
        } else {
            IrType::Any
        }
    } else {
        IrType::Any
    };

    let init = decl.init.as_ref().map(|expr| match &**expr {
        ast::Expr::Lit(ast::Lit::Num(n)) => IrExpression::Literal(IrLiteral::Int(n.value as i32)),
        ast::Expr::Lit(ast::Lit::Str(s)) => {
            IrExpression::Literal(IrLiteral::Str(s.value.to_string()))
        }
        _ => IrExpression::Identifier("unsupported".to_string()),
    });

    Some(IrVariable {
        name: name,
        ty: ty,
        value: init,
    })
}

fn fn_decl_to_ir(fn_decl: &ast::FnDecl) -> Option<IrFunction> {
    /*
        pub name: String,
        pub params: Vec<IrParam>,
        pub body: Vec<IrStmt>,
    */
    let name: String = fn_decl.ident.sym.to_string();

    let mut params: Vec<IrParam> = Vec::new();
    for p in &fn_decl.function.params {
        match &p.pat {
            ast::Pat::Ident(ast::BindingIdent { id, type_ann }) => {
                let pname = id.sym.to_string();
                let pty = type_ann
                    .as_ref()
                    .map(|ann| ts_type_ann_to_ir(ann))
                    .unwrap_or(IrType::Any);
                params.push(IrParam {
                    name: pname,
                    ty: pty,
                });
            }
            _ => return None,
        }
    }
    Some(IrFunction {
        name,
        params,
        // ret: ret_ty,          // TODO: раскомментируйте, если в вашей IR есть поле возвращаемого типа
        body: Vec::new(), // TODO: замените на тип вашей IR (например, None)
    })
}

fn ts_type_ann_to_ir(ann: &ast::TsTypeAnn) -> IrType {
    match &*ann.type_ann {
        ast::TsType::TsKeywordType(keyword) => match keyword.kind {
            ast::TsKeywordTypeKind::TsStringKeyword => IrType::Str,
            ast::TsKeywordTypeKind::TsNumberKeyword => IrType::Int,
            ast::TsKeywordTypeKind::TsBooleanKeyword => IrType::Bool,
            _ => IrType::Any,
        },
        _ => IrType::Any,
    }
}