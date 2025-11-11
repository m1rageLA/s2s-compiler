use ir::{IrAssignOp, IrExpression};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};

use crate::Codegen;

use super::unsupported::unsupported_assign_op;

pub(crate) fn assignment_tokens(
    op: IrAssignOp,
    left: &IrExpression,
    right: &IrExpression,
) -> TokenStream {
    let left_tokens = left.codegen();
    let right_tokens = right.codegen();

    let target_ident = format_ident!("ts_2_rs_target", span = Span::mixed_site());
    let value_ident = format_ident!("ts_2_rs_value", span = Span::mixed_site());
    let rhs_ident = format_ident!("ts_2_rs_rhs", span = Span::mixed_site());

    match op {
        IrAssignOp::Assign => quote!({
            let #value_ident = (#right_tokens).clone();
            let #target_ident = &mut #left_tokens;
            *#target_ident = (#value_ident).clone();
            #value_ident
        }),
        IrAssignOp::AddAssign => value_compound_assignment(
            quote!(runtime::value::ops::add),
            &target_ident,
            &rhs_ident,
            &left_tokens,
            &right_tokens,
        ),
        IrAssignOp::SubAssign => value_compound_assignment(
            quote!(runtime::value::ops::sub),
            &target_ident,
            &rhs_ident,
            &left_tokens,
            &right_tokens,
        ),
        IrAssignOp::MulAssign => value_compound_assignment(
            quote!(runtime::value::ops::mul),
            &target_ident,
            &rhs_ident,
            &left_tokens,
            &right_tokens,
        ),
        IrAssignOp::DivAssign => value_compound_assignment(
            quote!(runtime::value::ops::div),
            &target_ident,
            &rhs_ident,
            &left_tokens,
            &right_tokens,
        ),
        IrAssignOp::ModAssign => value_compound_assignment(
            quote!(runtime::value::ops::modulo),
            &target_ident,
            &rhs_ident,
            &left_tokens,
            &right_tokens,
        ),
        IrAssignOp::LeftShiftAssign => bitwise_assignment(
            quote!(<<),
            &target_ident,
            &rhs_ident,
            &left_tokens,
            &right_tokens,
        ),
        IrAssignOp::RightShiftAssign => bitwise_assignment(
            quote!(>>),
            &target_ident,
            &rhs_ident,
            &left_tokens,
            &right_tokens,
        ),
        IrAssignOp::BitwiseOrAssign => bitwise_assignment(
            quote!(|),
            &target_ident,
            &rhs_ident,
            &left_tokens,
            &right_tokens,
        ),
        IrAssignOp::BitwiseXorAssign => bitwise_assignment(
            quote!(^),
            &target_ident,
            &rhs_ident,
            &left_tokens,
            &right_tokens,
        ),
        IrAssignOp::BitwiseAndAssign => bitwise_assignment(
            quote!(&),
            &target_ident,
            &rhs_ident,
            &left_tokens,
            &right_tokens,
        ),
        IrAssignOp::ExpAssign => {
            exponent_assign_tokens(&target_ident, &rhs_ident, &left_tokens, &right_tokens)
        }
        IrAssignOp::UnsignedRightShiftAssign => unsupported_assign_op("unsigned right shift"),
        IrAssignOp::LogicalOrAssign => unsupported_assign_op("logical or assignment"),
        IrAssignOp::LogicalAndAssign => unsupported_assign_op("logical and assignment"),
        IrAssignOp::NullishCoalesceAssign => unsupported_assign_op("nullish coalesce assignment"),
    }
}

fn value_compound_assignment(
    op_fn: TokenStream,
    target_ident: &proc_macro2::Ident,
    rhs_ident: &proc_macro2::Ident,
    left_tokens: &TokenStream,
    right_tokens: &TokenStream,
) -> TokenStream {
    quote!({
        let #rhs_ident = (#right_tokens).clone();
        let #target_ident = &mut #left_tokens;
        let ts_2_rs_new = #op_fn((*#target_ident).clone(), (#rhs_ident).clone());
        *#target_ident = ts_2_rs_new.clone();
        ts_2_rs_new
    })
}

fn exponent_assign_tokens(
    target_ident: &proc_macro2::Ident,
    rhs_ident: &proc_macro2::Ident,
    left_tokens: &TokenStream,
    right_tokens: &TokenStream,
) -> TokenStream {
    quote!({
        let #rhs_ident = (#right_tokens).clone();
        let #target_ident = &mut #left_tokens;
        let ts_2_rs_base = (*#target_ident).clone().into_number();
        let ts_2_rs_exp = (#rhs_ident).clone().into_number();
        let ts_2_rs_new = runtime::value::Value::Number(ts_2_rs_base.powf(ts_2_rs_exp));
        *#target_ident = ts_2_rs_new.clone();
        ts_2_rs_new
    })
}

fn bitwise_assignment(
    operator: TokenStream,
    target_ident: &proc_macro2::Ident,
    rhs_ident: &proc_macro2::Ident,
    left_tokens: &TokenStream,
    right_tokens: &TokenStream,
) -> TokenStream {
    quote!({
        let #rhs_ident = #right_tokens;
        let #target_ident = &mut #left_tokens;
        let ts_2_rs_lhs = (*#target_ident).clone().into_number() as i64;
        let ts_2_rs_rhs = (#rhs_ident).clone().into_number() as i64;
        let ts_2_rs_new = runtime::value::Value::Number(((ts_2_rs_lhs) #operator (ts_2_rs_rhs)) as f64);
        *#target_ident = ts_2_rs_new.clone();
        ts_2_rs_new
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use ir::{IrExpression, IrLiteral};
    use quote::quote;

    #[test]
    fn simple_assignment_returns_assigned_value() {
        let tokens = assignment_tokens(
            IrAssignOp::Assign,
            &IrExpression::Identifier("value".into()),
            &IrExpression::Literal(IrLiteral::Number(5.0)),
        );

        let expected = quote!({
            let ts_2_rs_value = (runtime::value::Value::Number(5.0)).clone();
            let ts_2_rs_target = &mut value;
            *ts_2_rs_target = (ts_2_rs_value).clone();
            ts_2_rs_value
        });

        assert_eq!(tokens.to_string(), expected.to_string());
    }

    #[test]
    fn add_assign_updates_and_returns_new_value() {
        let tokens = assignment_tokens(
            IrAssignOp::AddAssign,
            &IrExpression::Identifier("counter".into()),
            &IrExpression::Literal(IrLiteral::Number(2.0)),
        );

        let expected = quote!({
            let ts_2_rs_rhs = (runtime::value::Value::Number(2.0)).clone();
            let ts_2_rs_target = &mut counter;
            let ts_2_rs_new =
                runtime::value::ops::add((*ts_2_rs_target).clone(), (ts_2_rs_rhs).clone());
            *ts_2_rs_target = ts_2_rs_new.clone();
            ts_2_rs_new
        });

        assert_eq!(tokens.to_string(), expected.to_string());
    }

    #[test]
    fn exponent_assign_is_translated_using_powf() {
        let tokens = assignment_tokens(
            IrAssignOp::ExpAssign,
            &IrExpression::Identifier("base".into()),
            &IrExpression::Literal(IrLiteral::Number(3.0)),
        );

        let expected = quote!({
            let ts_2_rs_rhs = (runtime::value::Value::Number(3.0)).clone();
            let ts_2_rs_target = &mut base;
            let ts_2_rs_base = (*ts_2_rs_target).clone().into_number();
            let ts_2_rs_exp = (ts_2_rs_rhs).clone().into_number();
            let ts_2_rs_new = runtime::value::Value::Number(ts_2_rs_base.powf(ts_2_rs_exp));
            *ts_2_rs_target = ts_2_rs_new.clone();
            ts_2_rs_new
        });

        assert_eq!(tokens.to_string(), expected.to_string());
    }
}
