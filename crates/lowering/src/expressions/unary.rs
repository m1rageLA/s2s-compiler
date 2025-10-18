use super::expr_to_ir;
use super::*;

pub fn unary_expr_to_ir(u: &ast::UnaryExpr) -> IrExpression {
    let inner = expr_to_ir(&u.arg);
    match u.op {
        ast::UnaryOp::Minus => match inner {
            IrExpression::Literal(IrLiteral::Number(value)) => {
                IrExpression::Literal(IrLiteral::Number(-value))
            }
            _ => IrExpression::Binary {
                op: IrBinOp::Sub,
                left: Box::new(IrExpression::Literal(IrLiteral::Number(0.0))),
                right: Box::new(inner),
            },
        },
        ast::UnaryOp::Plus => inner,
        _ => IrExpression::Identifier("unsupported_unary".to_string()),
    }
}

pub(crate) fn paren_to_ir(p: &ast::ParenExpr) -> IrExpression {
    expr_to_ir(&p.expr)
}
