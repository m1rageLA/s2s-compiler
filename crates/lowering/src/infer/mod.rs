use ir::{IrExpression, IrType};

mod expression_binary;
mod expression_conditional;
mod expression_identifier;
mod expression_literal;
mod expression_runtime;
mod expression_template;
mod expression_trivial;
mod function;
mod statements;
mod unify;

pub(crate) use function::infer_function_return_type;
pub(crate) use statements::collect_return_types;
pub(crate) use unify::unify_type;

pub(crate) fn infer_expression_type(expr: &IrExpression) -> Option<IrType> {
    match expr {
        IrExpression::Literal(literal) => expression_literal::infer_literal(literal),
        IrExpression::Identifier(name) => expression_identifier::infer_identifier(name),
        IrExpression::Binary { op, left, right } => {
            expression_binary::infer_binary(*op, left, right)
        }
        IrExpression::Conditional {
            consequent,
            alternate,
            ..
        } => expression_conditional::infer_conditional(consequent, alternate),
        IrExpression::Template(parts) => expression_template::infer_template(parts),
        IrExpression::RuntimeCall(call) => expression_runtime::infer_runtime(call),
        _ => expression_trivial::infer_default(expr),
    }
}
