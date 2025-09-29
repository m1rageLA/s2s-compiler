use ir::{IrItem, IrModule};
use swc_ecma_ast::{self as ast};

use crate::declarations::{fn_decl_to_ir, var_decl_to_ir};
use crate::expressions::expr_to_ir;
use crate::statements::block_to_ir;

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
            ast::ModuleItem::Stmt(ast::Stmt::Expr(expr_stmt)) => {
                let ir_expr = expr_to_ir(&expr_stmt.expr);
                items.push(IrItem::Expression(ir_expr));
            }
            ast::ModuleItem::Stmt(ast::Stmt::Block(block)) => {
                let ir_block = block_to_ir(block);
                items.push(IrItem::Block(ir_block));
            }
            _ => {}
        }
    }

    IrModule { items }
}
