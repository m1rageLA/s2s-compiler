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
    Some(IrVariable {
        name: "АААА".to_string(),
        ty: IrType::Int(0),
        value: None,
    })
}
