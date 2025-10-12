use super::*;

pub(crate) fn array_expr_to_ir(a: &ast::ArrayLit) -> IrExpression {
    IrExpression::Array(
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
    )
}
