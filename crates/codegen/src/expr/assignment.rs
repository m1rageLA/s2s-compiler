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
            let #value_ident = #right_tokens;
            let #target_ident = &mut #left_tokens;
            *#target_ident = (#value_ident).clone();
            #value_ident
        }),
        IrAssignOp::AddAssign => simple_compound_assignment(
            quote!(+=),
            &target_ident,
            &rhs_ident,
            &left_tokens,
            &right_tokens,
        ),
        IrAssignOp::SubAssign => simple_compound_assignment(
            quote!(-=),
            &target_ident,
            &rhs_ident,
            &left_tokens,
            &right_tokens,
        ),
        IrAssignOp::MulAssign => simple_compound_assignment(
            quote!(*=),
            &target_ident,
            &rhs_ident,
            &left_tokens,
            &right_tokens,
        ),
        IrAssignOp::DivAssign => simple_compound_assignment(
            quote!(/=),
            &target_ident,
            &rhs_ident,
            &left_tokens,
            &right_tokens,
        ),
        IrAssignOp::ModAssign => simple_compound_assignment(
            quote!(%=),
            &target_ident,
            &rhs_ident,
            &left_tokens,
            &right_tokens,
        ),
        IrAssignOp::LeftShiftAssign => simple_compound_assignment(
            quote!(<<=),
            &target_ident,
            &rhs_ident,
            &left_tokens,
            &right_tokens,
        ),
        IrAssignOp::RightShiftAssign => simple_compound_assignment(
            quote!(>>=),
            &target_ident,
            &rhs_ident,
            &left_tokens,
            &right_tokens,
        ),
        IrAssignOp::BitwiseOrAssign => simple_compound_assignment(
            quote!(|=),
            &target_ident,
            &rhs_ident,
            &left_tokens,
            &right_tokens,
        ),
        IrAssignOp::BitwiseXorAssign => simple_compound_assignment(
            quote!(^=),
            &target_ident,
            &rhs_ident,
            &left_tokens,
            &right_tokens,
        ),
        IrAssignOp::BitwiseAndAssign => simple_compound_assignment(
            quote!(&=),
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

fn simple_compound_assignment(
    operator: TokenStream,
    target_ident: &proc_macro2::Ident,
    rhs_ident: &proc_macro2::Ident,
    left_tokens: &TokenStream,
    right_tokens: &TokenStream,
) -> TokenStream {
    quote!({
        let #rhs_ident = #right_tokens;
        let #target_ident = &mut #left_tokens;
        *#target_ident #operator #rhs_ident;
        (*#target_ident).clone()
    })
}

fn exponent_assign_tokens(
    target_ident: &proc_macro2::Ident,
    rhs_ident: &proc_macro2::Ident,
    left_tokens: &TokenStream,
    right_tokens: &TokenStream,
) -> TokenStream {
    quote!({
        let #rhs_ident = #right_tokens;
        let #target_ident = &mut #left_tokens;
        *#target_ident = (*#target_ident).powf(#rhs_ident.clone());
        (*#target_ident).clone()
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
            let ts_2_rs_value = 5.0;
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
            let ts_2_rs_rhs = 2.0;
            let ts_2_rs_target = &mut counter;
            *ts_2_rs_target += ts_2_rs_rhs;
            (*ts_2_rs_target).clone()
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
            let ts_2_rs_rhs = 3.0;
            let ts_2_rs_target = &mut base;
            *ts_2_rs_target = (*ts_2_rs_target).powf(ts_2_rs_rhs.clone());
            (*ts_2_rs_target).clone()
        });

        assert_eq!(tokens.to_string(), expected.to_string());
    }
}
