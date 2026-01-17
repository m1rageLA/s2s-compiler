use ir::{IrAssignOp, IrExpression, IrPostfixOp, IrPrefixOp, IrType};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{typing, Codegen};

pub fn expression_stmt_tokens(expr: &IrExpression) -> TokenStream {
    if let IrExpression::Sequence(exprs) = expr {
        let tokens: Vec<TokenStream> = exprs.iter().map(expression_stmt_tokens).collect();
        return quote! { #(#tokens)* };
    }

    if let Some(optimized) = optimize_statement_expr(expr) {
        return optimized;
    }

    let expr_tokens = expr.codegen();
    quote! { #expr_tokens; }
}

fn optimize_statement_expr(expr: &IrExpression) -> Option<TokenStream> {
    match expr {
        IrExpression::Assignment { op, left, right } => {
            optimize_assignment_stmt(*op, left.as_ref(), right.as_ref())
        }
        IrExpression::PrefixUnary { arg, op } => optimize_prefix_stmt(arg.as_ref(), *op),
        IrExpression::PostfixUnary { left, op } => optimize_postfix_stmt(left.as_ref(), *op),
        _ => None,
    }
}

fn optimize_assignment_stmt(
    op: IrAssignOp,
    left: &IrExpression,
    right: &IrExpression,
) -> Option<TokenStream> {
    // Only optimize simple identifiers with known static type.
    let ident = match left {
        IrExpression::Identifier(name) => format_ident!("{}", name),
        _ => return None,
    };

    let left_ty = typing::infer_expression_type(left)?;
    if matches!(left_ty, IrType::Any | IrType::Value) {
        return None;
    }

    let right_tokens = right.codegen();
    let coerced_rhs = typing::coerce_to_type(
        quote! { (#right_tokens) },
        &left_ty,
        typing::infer_expression_type(right),
    );

    let tokens = match op {
        IrAssignOp::Assign => quote!({ #ident = #coerced_rhs; }),
        IrAssignOp::AddAssign => match left_ty {
            IrType::Str => quote!({
                #ident.push_str(&(#coerced_rhs));
            }),
            _ => quote!({
                #ident = #ident + #coerced_rhs;
            }),
        },
        IrAssignOp::SubAssign => quote!({ #ident = #ident - #coerced_rhs; }),
        IrAssignOp::MulAssign => quote!({ #ident = #ident * #coerced_rhs; }),
        IrAssignOp::DivAssign => quote!({ #ident = #ident / #coerced_rhs; }),
        IrAssignOp::ModAssign => quote!({ #ident = #ident % #coerced_rhs; }),
        _ => return None,
    };

    Some(tokens)
}

fn optimize_prefix_stmt(arg: &IrExpression, op: IrPrefixOp) -> Option<TokenStream> {
    let ident = match arg {
        IrExpression::Identifier(name) => format_ident!("{}", name),
        _ => return None,
    };

    let ty = typing::infer_expression_type(arg)?;
    match ty {
        IrType::Number | IrType::UInt => {}
        _ => return None,
    }

    let tokens = match op {
        IrPrefixOp::Increment => match ty {
            IrType::UInt => quote!({ #ident += 1usize; }),
            _ => quote!({ #ident += 1.0; }),
        },
        IrPrefixOp::Decrement => match ty {
            IrType::UInt => quote!({ #ident = #ident.saturating_sub(1usize); }),
            _ => quote!({ #ident -= 1.0; }),
        },
    };
    Some(tokens)
}

fn optimize_postfix_stmt(left: &IrExpression, op: IrPostfixOp) -> Option<TokenStream> {
    // In statement position, postfix ++/-- is equivalent to prefix for side effects.
    optimize_prefix_stmt(left, match op {
        IrPostfixOp::Increment => IrPrefixOp::Increment,
        IrPostfixOp::Decrement => IrPrefixOp::Decrement,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use ir::{IrAssignOp, IrLiteral, IrPostfixOp, IrType};
    use crate::typing;
    use quote::quote;

    #[test]
    fn wraps_expression_with_semicolon() {
        let expr = IrExpression::Identifier("value".into());
        let tokens = expression_stmt_tokens(&expr);
        assert_eq!(tokens.to_string(), quote! { value; }.to_string());
    }

    #[test]
    fn optimizes_simple_add_assign_statement() {
        typing::reset();
        typing::push_scope();
        typing::define("m", IrType::Number);
        let expr = IrExpression::Assignment {
            op: IrAssignOp::AddAssign,
            left: Box::new(IrExpression::Identifier("m".into())),
            right: Box::new(IrExpression::Literal(IrLiteral::Number(1.0))),
        };
        let tokens = expression_stmt_tokens(&expr);
        assert_eq!(
            tokens.to_string(),
            quote!({ m = m + ((1)) as f64; }).to_string()
        );
    }

    #[test]
    fn optimizes_postfix_increment_statement() {
        typing::reset();
        typing::push_scope();
        typing::define("i", IrType::Number);
        let expr = IrExpression::PostfixUnary {
            left: Box::new(IrExpression::Identifier("i".into())),
            op: IrPostfixOp::Increment,
        };
        let tokens = expression_stmt_tokens(&expr);
        assert_eq!(tokens.to_string(), quote!({ i += 1.0; }).to_string());
    }

    #[test]
    fn optimizes_postfix_increment_statement_for_uint() {
        typing::reset();
        typing::push_scope();
        typing::define("i", IrType::UInt);
        let expr = IrExpression::PostfixUnary {
            left: Box::new(IrExpression::Identifier("i".into())),
            op: IrPostfixOp::Increment,
        };
        let tokens = expression_stmt_tokens(&expr);
        assert_eq!(tokens.to_string(), quote!({ i += 1usize; }).to_string());
    }
}
