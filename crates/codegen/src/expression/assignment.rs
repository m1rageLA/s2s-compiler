use ir::{IrAssignOp, IrExpression};
use proc_macro2::{Literal, Span, TokenStream};
use quote::{format_ident, quote};

use crate::{Codegen, typing};

use super::unsupported::unsupported_assign_op;

pub(crate) fn assignment_tokens(
    op: IrAssignOp,
    left: &IrExpression,
    right: &IrExpression,
) -> TokenStream {
    if let IrExpression::Member { object, property } = left {
        return member_assignment_tokens(op, object.as_ref(), property, right);
    }

    let left_tokens = left.codegen();
    let right_tokens = right.codegen();

    let target_ident = format_ident!("ts_2_rs_target", span = Span::mixed_site());
    let value_ident = format_ident!("ts_2_rs_value", span = Span::mixed_site());
    let rhs_ident = format_ident!("ts_2_rs_rhs", span = Span::mixed_site());

    let left_ty = typing::infer_expression_type(left);
    let right_ty = typing::infer_expression_type(right);
    let dynamic = matches!(left_ty, Some(ir::IrType::Any | ir::IrType::Value)) || left_ty.is_none();
    let coerce_rhs = |tokens: TokenStream| {
        typing::coerce_to_type(tokens, &left_ty.unwrap_or(ir::IrType::Value), right_ty)
    };

    match op {
        IrAssignOp::Assign => {
            if dynamic {
                quote!({
                    let #value_ident = (#right_tokens).clone();
                    let #target_ident = &mut #left_tokens;
                    *#target_ident = (#value_ident).clone();
                    #value_ident
                })
            } else {
                let coerced = coerce_rhs(quote! { (#right_tokens) });
                quote!({
                    let #value_ident = #coerced;
                    let #target_ident = &mut #left_tokens;
                    *#target_ident = (#value_ident).clone();
                    #value_ident
                })
            }
        }
        IrAssignOp::AddAssign => {
            if dynamic {
                value_compound_assignment(
                    quote!(runtime::value::ops::add),
                    &target_ident,
                    &rhs_ident,
                    &left_tokens,
                    &right_tokens,
                )
            } else if matches!(left_ty, Some(ir::IrType::Str)) {
                quote!({
                    let #rhs_ident = (#right_tokens).to_string();
                    let #target_ident = &mut #left_tokens;
                    #target_ident.push_str(&#rhs_ident);
                    (#target_ident).clone()
                })
            } else {
                let coerced_rhs = coerce_rhs(quote! { (#right_tokens) });
                quote!({
                    let #rhs_ident = #coerced_rhs;
                    let #target_ident = &mut #left_tokens;
                    *#target_ident = (*#target_ident) + (#rhs_ident);
                    (*#target_ident)
                })
            }
        }
        IrAssignOp::SubAssign => {
            if dynamic {
                value_compound_assignment(
                    quote!(runtime::value::ops::sub),
                    &target_ident,
                    &rhs_ident,
                    &left_tokens,
                    &right_tokens,
                )
            } else {
                let coerced_rhs = coerce_rhs(quote! { (#right_tokens) });
                quote!({
                    let #rhs_ident = #coerced_rhs;
                    let #target_ident = &mut #left_tokens;
                    *#target_ident = (*#target_ident) - (#rhs_ident);
                    (*#target_ident)
                })
            }
        }
        IrAssignOp::MulAssign => {
            if dynamic {
                value_compound_assignment(
                    quote!(runtime::value::ops::mul),
                    &target_ident,
                    &rhs_ident,
                    &left_tokens,
                    &right_tokens,
                )
            } else {
                let coerced_rhs = coerce_rhs(quote! { (#right_tokens) });
                quote!({
                    let #rhs_ident = #coerced_rhs;
                    let #target_ident = &mut #left_tokens;
                    *#target_ident = (*#target_ident) * (#rhs_ident);
                    (*#target_ident)
                })
            }
        }
        IrAssignOp::DivAssign => {
            if dynamic {
                value_compound_assignment(
                    quote!(runtime::value::ops::div),
                    &target_ident,
                    &rhs_ident,
                    &left_tokens,
                    &right_tokens,
                )
            } else {
                let coerced_rhs = coerce_rhs(quote! { (#right_tokens) });
                quote!({
                    let #rhs_ident = #coerced_rhs;
                    let #target_ident = &mut #left_tokens;
                    *#target_ident = (*#target_ident) / (#rhs_ident);
                    (*#target_ident)
                })
            }
        }
        IrAssignOp::ModAssign => {
            if dynamic {
                value_compound_assignment(
                    quote!(runtime::value::ops::modulo),
                    &target_ident,
                    &rhs_ident,
                    &left_tokens,
                    &right_tokens,
                )
            } else {
                let coerced_rhs = coerce_rhs(quote! { (#right_tokens) });
                quote!({
                    let #rhs_ident = #coerced_rhs;
                    let #target_ident = &mut #left_tokens;
                    *#target_ident = (*#target_ident) % (#rhs_ident);
                    (*#target_ident)
                })
            }
        }
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

fn member_assignment_tokens(
    op: IrAssignOp,
    object: &IrExpression,
    property: &str,
    right: &IrExpression,
) -> TokenStream {
    match op {
        IrAssignOp::Assign => member_simple_assign(object, property, right),
        IrAssignOp::AddAssign => member_value_op(quote!(runtime::value::ops::add), object, property, right),
        IrAssignOp::SubAssign => member_value_op(quote!(runtime::value::ops::sub), object, property, right),
        IrAssignOp::MulAssign => member_value_op(quote!(runtime::value::ops::mul), object, property, right),
        IrAssignOp::DivAssign => member_value_op(quote!(runtime::value::ops::div), object, property, right),
        IrAssignOp::ModAssign => member_value_op(quote!(runtime::value::ops::modulo), object, property, right),
        IrAssignOp::ExpAssign => member_exponent_assign(object, property, right),
        IrAssignOp::LeftShiftAssign => member_bitwise_assign(quote!(<<), object, property, right),
        IrAssignOp::RightShiftAssign => member_bitwise_assign(quote!(>>), object, property, right),
        IrAssignOp::BitwiseOrAssign => member_bitwise_assign(quote!(|), object, property, right),
        IrAssignOp::BitwiseXorAssign => member_bitwise_assign(quote!(^), object, property, right),
        IrAssignOp::BitwiseAndAssign => member_bitwise_assign(quote!(&), object, property, right),
        IrAssignOp::UnsignedRightShiftAssign
        | IrAssignOp::LogicalOrAssign
        | IrAssignOp::LogicalAndAssign
        | IrAssignOp::NullishCoalesceAssign => unsupported_assign_op("unsupported member assignment"),
    }
}

fn member_simple_assign(object: &IrExpression, property: &str, right: &IrExpression) -> TokenStream {
    let object_tokens = object.codegen();
    let right_tokens = right.codegen();
    let property_literal = Literal::string(property);

    quote!({
        let ts_2_rs_value = (#right_tokens).clone();
        let ts_2_rs_target = &mut #object_tokens;
        runtime::value::ops::set_property_in_place(ts_2_rs_target, #property_literal, ts_2_rs_value.clone());
        ts_2_rs_value
    })
}

fn member_value_op(
    op_fn: TokenStream,
    object: &IrExpression,
    property: &str,
    right: &IrExpression,
) -> TokenStream {
    let object_tokens = object.codegen();
    let right_tokens = right.codegen();
    let property_literal = Literal::string(property);
    let property_literal_for_set = property_literal.clone();

    quote!({
        let ts_2_rs_rhs = (#right_tokens).clone();
        let ts_2_rs_target = &mut #object_tokens;
        let ts_2_rs_current = runtime::value::ops::get_property((*ts_2_rs_target).clone(), #property_literal);
        let ts_2_rs_new = #op_fn(ts_2_rs_current, (ts_2_rs_rhs).clone());
        runtime::value::ops::set_property_in_place(ts_2_rs_target, #property_literal_for_set, ts_2_rs_new.clone());
        ts_2_rs_new
    })
}

fn member_exponent_assign(
    object: &IrExpression,
    property: &str,
    right: &IrExpression,
) -> TokenStream {
    let object_tokens = object.codegen();
    let right_tokens = right.codegen();
    let property_literal = Literal::string(property);
    let property_literal_for_set = property_literal.clone();

    quote!({
        let ts_2_rs_rhs = (#right_tokens).clone();
        let ts_2_rs_target = &mut #object_tokens;
        let ts_2_rs_base = runtime::value::ops::get_property((*ts_2_rs_target).clone(), #property_literal).into_number();
        let ts_2_rs_exp = (ts_2_rs_rhs).clone().into_number();
        let ts_2_rs_new = runtime::value::Value::Number(ts_2_rs_base.powf(ts_2_rs_exp));
        runtime::value::ops::set_property_in_place(ts_2_rs_target, #property_literal_for_set, ts_2_rs_new.clone());
        ts_2_rs_new
    })
}

fn member_bitwise_assign(
    operator: TokenStream,
    object: &IrExpression,
    property: &str,
    right: &IrExpression,
) -> TokenStream {
    let object_tokens = object.codegen();
    let right_tokens = right.codegen();
    let property_literal = Literal::string(property);
    let property_literal_for_set = property_literal.clone();

    quote!({
        let ts_2_rs_rhs = (#right_tokens).clone();
        let ts_2_rs_target = &mut #object_tokens;
        let ts_2_rs_lhs = runtime::value::ops::get_property((*ts_2_rs_target).clone(), #property_literal).into_number() as i64;
        let ts_2_rs_rhs_num = (ts_2_rs_rhs).clone().into_number() as i64;
        let ts_2_rs_new = runtime::value::Value::Number(((ts_2_rs_lhs) #operator (ts_2_rs_rhs_num)) as f64);
        runtime::value::ops::set_property_in_place(ts_2_rs_target, #property_literal_for_set, ts_2_rs_new.clone());
        ts_2_rs_new
    })
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
            let ts_2_rs_value = (5.0).clone();
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
            let ts_2_rs_rhs = (2.0).clone();
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
            let ts_2_rs_rhs = (3.0).clone();
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
