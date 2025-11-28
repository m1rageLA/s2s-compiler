use crate::runtime::runtime_call_tokens;
use ir::IrExpression;
use proc_macro2::TokenStream;

use crate::Codegen;
mod array;
mod arrow;
mod assignment;
mod binary;
mod call;
mod conditional;
mod function;
mod identifier;
mod literal;
mod member;
mod object;
mod template;
mod unary;
mod unsupported;

use array::array_tokens;
use arrow::arrow_tokens;
use assignment::assignment_tokens;
use binary::binary_op_tokens;
use call::call_tokens;
use conditional::conditional_tokens;
use function::function_expr_tokens;
use identifier::identifier_tokens;
use member::member_tokens;
use object::object_literal_tokens;
use template::template_literal_tokens;
use unary::postfixunary_tokens;
use unsupported::unsupported_expr;

impl Codegen for IrExpression {
    type Output = TokenStream;

    fn codegen(&self) -> TokenStream {
        match self {
            IrExpression::Identifier(name) => identifier_tokens(name),
            IrExpression::Literal(literal) => literal.codegen(),
            IrExpression::Binary { op, left, right } => binary_op_tokens(*op, left, right),
            IrExpression::Assignment { op, left, right } => assignment_tokens(*op, left, right),
            IrExpression::Template(parts) => template_literal_tokens(parts),
            IrExpression::RuntimeCall(namespace) => runtime_call_tokens(namespace),
            IrExpression::Call { callee, args } => call_tokens(callee, args),
            IrExpression::Conditional {
                test,
                consequent,
                alternate,
            } => conditional_tokens(test, consequent, alternate),
            IrExpression::Array(items) => array_tokens(items),
            IrExpression::Object(properties) => object_literal_tokens(properties),
            IrExpression::Member { object, property } => member_tokens(object, property),
            IrExpression::SuperCall { .. } => unsupported_expr("super call"),
            IrExpression::Arrow { params, body } => arrow_tokens(params, body),
            IrExpression::PostfixUnary { left, op } => {
                postfixunary_tokens(left.clone(), op.clone())
            }
            IrExpression::Function(function) => function_expr_tokens(function),
            IrExpression::Paren(p) => p.codegen(),
            _ => unsupported_expr("unsupported expression"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ir::{IrAssignOp, IrExpression, IrLiteral, IrObjectProperty};

    #[test]
    fn identifier_expression_delegates_to_identifier_tokens() {
        let expr = IrExpression::Identifier("value".into());
        assert_eq!(
            expr.codegen().to_string(),
            identifier_tokens("value").to_string()
        );
    }

    #[test]
    fn array_expression_leverages_array_tokens() {
        let expr = IrExpression::Array(vec![
            IrExpression::Literal(IrLiteral::Number(1.0)),
            IrExpression::Literal(IrLiteral::Number(2.0)),
        ]);
        assert_eq!(
            expr.codegen().to_string(),
            array_tokens(&[
                IrExpression::Literal(IrLiteral::Number(1.0)),
                IrExpression::Literal(IrLiteral::Number(2.0))
            ])
            .to_string()
        );
    }

    #[test]
    fn object_expression_leverages_object_literal_tokens() {
        let expr = IrExpression::Object(vec![
            IrObjectProperty {
                key: "a".into(),
                value: IrExpression::Literal(IrLiteral::Number(1.0)),
            },
            IrObjectProperty {
                key: "b".into(),
                value: IrExpression::Identifier("value".into()),
            },
        ]);

        assert_eq!(
            expr.codegen().to_string(),
            object_literal_tokens(&[
                IrObjectProperty {
                    key: "a".into(),
                    value: IrExpression::Literal(IrLiteral::Number(1.0)),
                },
                IrObjectProperty {
                    key: "b".into(),
                    value: IrExpression::Identifier("value".into()),
                }
            ])
            .to_string()
        );
    }

    #[test]
    fn assignment_expression_leverages_assignment_tokens() {
        let expr = IrExpression::Assignment {
            op: IrAssignOp::AddAssign,
            left: Box::new(IrExpression::Identifier("counter".into())),
            right: Box::new(IrExpression::Literal(IrLiteral::Number(1.0))),
        };

        assert_eq!(
            expr.codegen().to_string(),
            assignment_tokens(
                IrAssignOp::AddAssign,
                &IrExpression::Identifier("counter".into()),
                &IrExpression::Literal(IrLiteral::Number(1.0))
            )
            .to_string()
        );
    }

    #[test]
    fn super_call_is_reported_as_unsupported() {
        let expr = IrExpression::SuperCall { args: vec![] };
        assert_eq!(
            expr.codegen().to_string(),
            unsupported_expr("super call").to_string()
        );
    }

    #[test]
    fn unknown_expression_routes_to_generic_unsupported_handler() {
        let expr = IrExpression::ArrayExpr(Vec::new());
        assert_eq!(
            expr.codegen().to_string(),
            unsupported_expr("unsupported expression").to_string()
        );
    }
}
