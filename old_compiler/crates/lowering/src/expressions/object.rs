use crate::expressions::{IrExpression, ast, coerce_to_value, expr_to_ir};
use ir::IrObjectProperty;

pub(crate) fn object_expr_to_ir(object: &ast::ObjectLit) -> IrExpression {
    let mut properties = Vec::new();

    for prop in &object.props {
        match prop {
            ast::PropOrSpread::Prop(boxed_prop) => match &**boxed_prop {
                ast::Prop::KeyValue(kv) => {
                    let key = match prop_name_to_string(&kv.key) {
                        Some(key) => key,
                        None => return IrExpression::Identifier("unsupported".to_string()),
                    };
                    let value = coerce_to_value(expr_to_ir(kv.value.as_ref()));
                    properties.push(IrObjectProperty { key, value });
                }
                ast::Prop::Shorthand(ident) => {
                    let name = ident.sym.to_string();
                    properties.push(IrObjectProperty {
                        key: name.clone(),
                        value: coerce_to_value(IrExpression::Identifier(name)),
                    });
                }
                _ => return IrExpression::Identifier("unsupported".to_string()),
            },
            ast::PropOrSpread::Spread(_) => {
                return IrExpression::Identifier("unsupported".to_string());
            }
        }
    }

    IrExpression::Object(properties)
}

fn prop_name_to_string(name: &ast::PropName) -> Option<String> {
    match name {
        ast::PropName::Ident(ident) => Some(ident.sym.to_string()),
        ast::PropName::Str(string) => Some(string.value.to_string()),
        ast::PropName::Num(num) => Some(num.value.to_string()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expressions::ast;
    use ir::{IrExpression, IrLiteral};

    fn lower_expression(source: &str) -> IrExpression {
        let module = parser::ast(source);
        let decl = match module.body.first().expect("expected statement") {
            ast::ModuleItem::Stmt(ast::Stmt::Decl(ast::Decl::Var(var_decl))) => {
                var_decl.decls.first().expect("expected declarator")
            }
            other => panic!("expected variable declaration, got {other:?}"),
        };

        let init = decl.init.as_ref().expect("expected initializer").as_ref();
        expr_to_ir(init)
    }

    #[test]
    fn lowers_object_literal_with_key_value_and_shorthand() {
        let ir = lower_expression("const value = { a: 1, b };");
        let props = match ir {
            IrExpression::Object(props) => props,
            other => panic!("expected object expression, got {other:?}"),
        };

        assert_eq!(props.len(), 2);
        assert_eq!(props[0].key, "a");
        let first_value = match &props[0].value {
            IrExpression::RuntimeCall(ir::RuntimeNamespace::Value(ir::ValueCall::Coerce {
                expr,
            })) => expr.as_ref(),
            other => panic!("expected value coercion for property 'a', got {other:?}"),
        };
        assert_eq!(*first_value, IrExpression::Literal(IrLiteral::Number(1.0)));

        assert_eq!(props[1].key, "b");
        match &props[1].value {
            IrExpression::RuntimeCall(ir::RuntimeNamespace::Value(ir::ValueCall::Coerce {
                expr,
            })) => {
                assert_eq!(**expr, IrExpression::Identifier("b".to_string()));
            }
            other => panic!("expected value coercion for shorthand property, got {other:?}"),
        }
    }
}
