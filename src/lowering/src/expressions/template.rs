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
