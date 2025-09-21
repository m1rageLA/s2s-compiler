use ir::*;
use swc_ecma_ast::{self as ast};

//ENTRY POIN
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
            _ => (),
        }
    }
    IrModule { items }
}

//DECLARATIONS
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

    let init = expr_to_ir(decl.init.as_ref().unwrap());
    Some(IrVariable {
        name: name,
        ty: ty,
        value: Option::from(init),
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
    let ret_ty: IrType = match &fn_decl.function.return_type {
        Some(ann) => ts_type_ann_to_ir(ann),
        None => IrType::Any,
    };

    Some(IrFunction {
        name,
        params,
        ret: ret_ty,
        body: Vec::new(), // TODO: замените на тип вашей IR (например, None)
    })
}

//TYPES
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

fn expr_to_ir(expr: &ast::Expr) -> IrExpression {
    match expr {
        ast::Expr::Lit(ast::Lit::Num(n)) => IrExpression::Literal(IrLiteral::Int(n.value as i32)),
        ast::Expr::Lit(ast::Lit::Str(s)) => {
            IrExpression::Literal(IrLiteral::Str(s.value.to_string()))
        }
        ast::Expr::Ident(i) => IrExpression::Identifier(i.to_string()),
        ast::Expr::Bin(b) => IrExpression::Binary {
            op: bin_op_to_ir(&b.op),
            left: Box::new(expr_to_ir(&b.left)),
            right: Box::new(expr_to_ir(&b.right)),
        },
        _ => IrExpression::Identifier("unsupported".to_string()),
    }
}

fn bin_op_to_ir(op: &ast::BinaryOp) -> IrBinOp {
    match op {
        ast::BinaryOp::Add => IrBinOp::Add,
        ast::BinaryOp::Sub => IrBinOp::Sub,
        ast::BinaryOp::Mul => IrBinOp::Mul,
        ast::BinaryOp::Div => IrBinOp::Div,
        ast::BinaryOp::Mod => IrBinOp::Mod,
        ast::BinaryOp::Exp => IrBinOp::Exp,

        ast::BinaryOp::EqEq => IrBinOp::Equal,
        ast::BinaryOp::EqEqEq => IrBinOp::StrictEqual,
        ast::BinaryOp::NotEq => IrBinOp::NotEqual,
        ast::BinaryOp::NotEqEq => IrBinOp::StrictNotEqual,

        ast::BinaryOp::Lt => IrBinOp::LessThan,
        ast::BinaryOp::LtEq => IrBinOp::LessThanOrEqual,
        ast::BinaryOp::Gt => IrBinOp::GreaterThan,
        ast::BinaryOp::GtEq => IrBinOp::GreaterThanOrEqual,

        ast::BinaryOp::LShift => IrBinOp::LeftShift,
        ast::BinaryOp::RShift => IrBinOp::RightShift,
        ast::BinaryOp::ZeroFillRShift => IrBinOp::UnsignedRightShift,

        ast::BinaryOp::BitOr => IrBinOp::BitwiseOr,
        ast::BinaryOp::BitXor => IrBinOp::BitwiseXor,
        ast::BinaryOp::BitAnd => IrBinOp::BitwiseAnd,

        ast::BinaryOp::LogicalOr => IrBinOp::LogicalOr,
        ast::BinaryOp::LogicalAnd => IrBinOp::LogicalAnd,

        ast::BinaryOp::In => IrBinOp::In,
        ast::BinaryOp::InstanceOf => IrBinOp::InstanceOf,

        _ => IrBinOp::Unsupported,
    }
}
