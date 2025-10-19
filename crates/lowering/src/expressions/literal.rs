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

#[cfg(test)]
mod tests {
    use super::*;
    use swc_atoms::Atom;
    use swc_common::{SyntaxContext, DUMMY_SP};

    #[test]
    fn converts_numeric_literals() {
        let number = ast::Number {
            span: DUMMY_SP,
            value: 42.0,
            raw: None,
        };
        match number_lit_to_ir(&number) {
            IrExpression::Literal(IrLiteral::Number(value)) => assert_eq!(value, 42.0),
            other => panic!("expected numeric literal, got {other:?}"),
        }
    }

    #[test]
    fn converts_string_literals() {
        let literal = ast::Str {
            span: DUMMY_SP,
            value: Atom::from("hello"),
            raw: None,
        };
        match string_lit_to_ir(&literal) {
            IrExpression::Literal(IrLiteral::Str(value)) => assert_eq!(value, "hello"),
            other => panic!("expected string literal, got {other:?}"),
        }
    }

    #[test]
    fn converts_boolean_literals() {
        let literal = ast::Bool {
            span: DUMMY_SP,
            value: true,
        };
        match bool_lit_to_ir(&literal) {
            IrExpression::Literal(IrLiteral::Bool(value)) => assert!(value),
            other => panic!("expected boolean literal, got {other:?}"),
        }
    }

    #[test]
    fn converts_identifiers() {
        let ident = ast::Ident::new("value".into(), DUMMY_SP, SyntaxContext::empty());
        match ident_to_ir(&ident) {
            IrExpression::Identifier(name) => assert_eq!(name, "value"),
            other => panic!("expected identifier expression, got {other:?}"),
        }
    }
}
