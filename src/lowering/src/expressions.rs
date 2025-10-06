use ir::{IrBinOp, IrExpression, IrLiteral, IrTemplatePart, RuntimeNamespace};
use swc_ecma_ast::{self as ast};

pub(crate) fn expr_to_ir(expr: &ast::Expr) -> IrExpression {
    match expr {
        ast::Expr::Lit(ast::Lit::Num(n)) => IrExpression::Literal(IrLiteral::Int(n.value as i32)),
        ast::Expr::Lit(ast::Lit::Str(s)) => {
            IrExpression::Literal(IrLiteral::Str(s.value.to_string()))
        }
        ast::Expr::Lit(ast::Lit::Bool(b)) => IrExpression::Literal(IrLiteral::Bool(b.value)),
        ast::Expr::Ident(i) => IrExpression::Identifier(i.sym.to_string()),
        ast::Expr::Paren(p) => expr_to_ir(&p.expr),
        ast::Expr::Bin(b) => IrExpression::Binary {
            op: bin_op_to_ir(&b.op),
            left: Box::new(expr_to_ir(&b.left)),
            right: Box::new(expr_to_ir(&b.right)),
        },
        ast::Expr::Call(call) => call_to_ir(call),
        ast::Expr::Array(a) => IrExpression::Array(
            a.elems
                .iter()
                .filter_map(|opt| opt.as_ref())
                .map(|expr_or_spread| match expr_or_spread {
                    ast::ExprOrSpread { spread: None, expr } => expr_to_ir(expr),
                    ast::ExprOrSpread {
                        spread: Some(_), ..
                    } => IrExpression::Identifier("spread_not_supported".to_string()),
                })
                .collect(),
        ),
        ast::Expr::Tpl(tpl) => IrExpression::Template(template_to_ir(tpl)),
        _ => IrExpression::Identifier("unsupported".to_string()),
    }
}

fn call_to_ir(c: &ast::CallExpr) -> IrExpression {
    match &c.callee {
        ast::Callee::Expr(expr) => {
            let callee = callee_to_ir(expr);
            let args = c.args.iter().map(|a| match a {
                ast::ExprOrSpread { spread: None, expr } => expr_to_ir(expr),
                _ => IrExpression::Identifier("spread_not_supported".to_string()),
            }).collect::<Vec<_>>();

            // Спец-случай: console.log(...)
            if let IrExpression::Member { object, property } = &callee {
                if matches!(**object, IrExpression::Identifier(ref s) if s == "console")
                    && property == "log"
                {
                    return IrExpression::RuntimeCall(RuntimeNamespace::Console(ir::ConsoleCall::Log(args)));
                }
            }

            IrExpression::Call {
                callee: Box::new(callee),
                args,
            }
        }

        ast::Callee::Super(_) => {
            let args = c.args.iter().map(|a| match a {
                ast::ExprOrSpread { spread: None, expr } => expr_to_ir(expr),
                _ => IrExpression::Identifier("spread_not_supported".to_string()),
            }).collect::<Vec<_>>();

            IrExpression::SuperCall { args }
        }

        ast::Callee::Import(_) => {
            IrExpression::Identifier("import_call_not_supported".to_string())
        }
    }
}

fn template_to_ir(tpl: &ast::Tpl) -> Vec<IrTemplatePart> {
    let mut parts = Vec::new();

    for (idx, quasi) in tpl.quasis.iter().enumerate() {
        let cooked = quasi
            .cooked
            .as_ref()
            .map(|atom| atom.to_string())
            .unwrap_or_else(|| quasi.raw.to_string());
        parts.push(IrTemplatePart::String(cooked));
        if let Some(expr) = tpl.exprs.get(idx) {
            parts.push(IrTemplatePart::Expr(Box::new(expr_to_ir(expr))));
        }
    }

    parts
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

pub(crate) fn bin_op_to_ir(op: &ast::BinaryOp) -> IrBinOp {
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
