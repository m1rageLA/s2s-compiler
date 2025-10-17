use ir::IrExpression;
use proc_macro2::TokenStream;

use crate::Codegen;
mod array;
mod arrow;
mod binary;
mod call;
mod conditional;
mod identifier;
mod literal;
mod member;
mod runtime;
mod template;
mod unsupported;
mod unary;
mod function;

use unary::postfixunary_tokens; 
use array::array_tokens;
use arrow::arrow_tokens;
use binary::binary_op_tokens;
use call::call_tokens;
use conditional::conditional_tokens;
use identifier::identifier_tokens;
use member::member_tokens;
use runtime::runtime_call_tokens;
use template::template_literal_tokens;
use unsupported::unsupported_expr;
use function::function_expr_tokens;

impl Codegen for IrExpression {
    type Output = TokenStream;


    fn codegen(&self) -> TokenStream {
        match self {
            IrExpression::Identifier(name) => identifier_tokens(name),
            IrExpression::Literal(literal) => literal.codegen(),
            IrExpression::Binary { op, left, right } => {
                let left_tokens = left.codegen();
                let right_tokens = right.codegen();
                binary_op_tokens(*op, left_tokens, right_tokens)
            }
            IrExpression::Template(parts) => template_literal_tokens(parts),
            IrExpression::RuntimeCall(namespace) => runtime_call_tokens(namespace),
            IrExpression::Call { callee, args } => call_tokens(callee, args),
            IrExpression::Conditional {
                test,
                consequent,
                alternate,
            } => conditional_tokens(test, consequent, alternate),
            IrExpression::Array(items) => array_tokens(items), 
            IrExpression::Member { object, property } => member_tokens(object, property),
            IrExpression::SuperCall { .. } => unsupported_expr("super call"),
            IrExpression::Arrow { params, body } => arrow_tokens(params, body),
            IrExpression::PostfixUnary { left, op } => postfixunary_tokens(left.clone(), op.clone()),
            IrExpression::Function(function) => function_expr_tokens(function),
            _ => unsupported_expr("unsupported expression"),
        }
    }
}
