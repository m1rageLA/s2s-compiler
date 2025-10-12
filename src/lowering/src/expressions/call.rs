use super::*;

pub fn call_to_ir(c: &ast::CallExpr) -> IrExpression {
    match &c.callee {
        ast::Callee::Expr(expr) => {
            let callee = callee_to_ir(expr);
            let args = c
                .args
                .iter()
                .map(|a| match a {
                    ast::ExprOrSpread { spread: None, expr } => expr_to_ir(expr),
                    _ => IrExpression::Identifier("spread_not_supported".to_string()),
                })
                .collect::<Vec<_>>();

            // Спец-случай: console.log(...)
            if let IrExpression::Member { object, property } = &callee {
                if matches!(**object, IrExpression::Identifier(ref s) if s == "console")
                    && property == "log"
                {
                    return IrExpression::RuntimeCall(RuntimeNamespace::Console(
                        ir::ConsoleCall::Log(args),
                    ));
                }
            }

            IrExpression::Call {
                callee: Box::new(callee),
                args,
            }
        }

        ast::Callee::Super(_) => {
            let args = c
                .args
                .iter()
                .map(|a| match a {
                    ast::ExprOrSpread { spread: None, expr } => expr_to_ir(expr),
                    _ => IrExpression::Identifier("spread_not_supported".to_string()),
                })
                .collect::<Vec<_>>();

            IrExpression::SuperCall { args }
        }

        ast::Callee::Import(_) => IrExpression::Identifier("import_call_not_supported".to_string()),
    }
}

fn callee_to_ir(expr: &ast::Expr) -> IrExpression {
    match expr {
        ast::Expr::Ident(i) => IrExpression::Identifier(i.sym.to_string()),
        ast::Expr::Member(m) => {
            let object = callee_to_ir(&m.obj);
            let property = match &m.prop {
                ast::MemberProp::Ident(ident) => ident.sym.to_string(),
                ast::MemberProp::PrivateName(_) => "private_not_supported".to_string(),
                ast::MemberProp::Computed(_) => "computed_not_supported".to_string(),
            };
            IrExpression::Member {
                object: Box::new(object),
                property,
            }
        }
        ast::Expr::SuperProp(prop) => {
            let property = match &prop.prop {
                ast::SuperProp::Ident(id) => id.sym.to_string(),
                ast::SuperProp::Computed(_) => "computed_not_supported".to_string(),
            };
            IrExpression::Member {
                object: Box::new(IrExpression::Identifier("super".to_string())),
                property,
            }
        }
        _ => IrExpression::Identifier("unsupported".to_string()),
    }
}
