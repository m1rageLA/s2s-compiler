use super::*;

pub(crate) fn template_to_ir(tpl: &ast::Tpl) -> Vec<IrTemplatePart> {
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

pub(crate) fn template_expr_to_ir(tpl: &ast::Tpl) -> IrExpression {
    IrExpression::Template(template_to_ir(tpl))
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{assert_identifier, expect_variable, lower};
    use ir::{IrExpression, IrTemplatePart};

    #[test]
    fn lowers_template_literals_with_expressions() {
        let ir_module = lower(
            r#"
            const message = `Hello ${name}!`;
        "#,
        );

        assert_eq!(ir_module.items.len(), 1);
        let variable = expect_variable(&ir_module.items[0], "message");
        let template = match variable
            .value
            .as_ref()
            .expect("message should have initializer")
        {
            IrExpression::Template(parts) => parts,
            other => panic!("expected template literal, got {other:?}"),
        };

        assert_eq!(template.len(), 3);
        match &template[0] {
            IrTemplatePart::String(value) => assert_eq!(value, "Hello "),
            other => panic!("expected leading string part, got {other:?}"),
        }
        match &template[1] {
            IrTemplatePart::Expr(expr) => assert_identifier(expr, "name"),
            other => panic!("expected interpolated expression, got {other:?}"),
        }
        match &template[2] {
            IrTemplatePart::String(value) => assert_eq!(value, "!"),
            other => panic!("expected trailing string part, got {other:?}"),
        }
    }
}
