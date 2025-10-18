use super::*;

pub(crate) fn number_lit_to_ir(n: &ast::Number) -> IrExpression {
    IrExpression::Literal(IrLiteral::Number(n.value))
}

pub(crate) fn string_lit_to_ir(s: &ast::Str) -> IrExpression {
    IrExpression::Literal(IrLiteral::Str(s.value.to_string()))
}

pub(crate) fn bool_lit_to_ir(b: &ast::Bool) -> IrExpression {
    IrExpression::Literal(IrLiteral::Bool(b.value))
}

pub(crate) fn ident_to_ir(i: &ast::Ident) -> IrExpression {
    IrExpression::Identifier(i.sym.to_string())
}
