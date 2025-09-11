use ir::*;
use swc_ecma_ast as ast;

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
    

    Some(IrVariable {
        name: name,
        ty: ty,
        value: None,
    })
}
