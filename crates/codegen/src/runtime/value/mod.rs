mod add;
mod coerce;
mod div;
mod equal;
mod greater_than;
mod greater_than_or_equal;
mod less_than;
mod less_than_or_equal;
mod mod_op;
mod mul;
mod not;
mod not_equal;
mod strict_equal;
mod strict_not_equal;
mod sub;

use crate::Codegen;
use add::add_tokens;
use coerce::coerce_tokens;
use div::div_tokens;
use equal::equal_tokens;
use greater_than::greater_than_tokens;
use greater_than_or_equal::greater_than_or_equal_tokens;
use ir::{IrExpression, ValueCall};
use less_than::less_than_tokens;
use less_than_or_equal::less_than_or_equal_tokens;
use mod_op::mod_tokens;
use mul::mul_tokens;
use not::logical_not_tokens;
use not_equal::not_equal_tokens;
use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};
use strict_equal::strict_equal_tokens;
use strict_not_equal::strict_not_equal_tokens;
use sub::sub_tokens;

pub(crate) fn value_call_tokens(call: &ValueCall) -> TokenStream {
    match call {
        ValueCall::Coerce { expr } => coerce_tokens(expr),
        ValueCall::Add { left, right } => add_tokens(left, right),
        ValueCall::Sub { left, right } => sub_tokens(left, right),
        ValueCall::Mul { left, right } => mul_tokens(left, right),
        ValueCall::Div { left, right } => div_tokens(left, right),
        ValueCall::Mod { left, right } => mod_tokens(left, right),
        ValueCall::Equal { left, right } => equal_tokens(left, right),
        ValueCall::StrictEqual { left, right } => strict_equal_tokens(left, right),
        ValueCall::NotEqual { left, right } => not_equal_tokens(left, right),
        ValueCall::StrictNotEqual { left, right } => strict_not_equal_tokens(left, right),
        ValueCall::LessThan { left, right } => less_than_tokens(left, right),
        ValueCall::LessThanOrEqual { left, right } => less_than_or_equal_tokens(left, right),
        ValueCall::GreaterThan { left, right } => greater_than_tokens(left, right),
        ValueCall::GreaterThanOrEqual { left, right } => greater_than_or_equal_tokens(left, right),
        ValueCall::LogicalNot { expr } => logical_not_tokens(expr),
        ValueCall::GetProperty { target, property } => get_property_tokens(target, property),
        ValueCall::GetPropertyDynamic { target, property } => {
            get_property_dynamic_tokens(target, property)
        }
    }
}

pub(super) fn binary_value_op(
    name: &str,
    left: &IrExpression,
    right: &IrExpression,
) -> TokenStream {
    let func = format_ident!("{}", name);
    let left_tokens = left.codegen();
    let right_tokens = right.codegen();
    quote! {{
        let left_tmp = (#left_tokens).clone();
        let right_tmp = (#right_tokens).clone();
        runtime::value::ops::#func(left_tmp, right_tmp)
    }}
}

pub(super) fn binary_bool_op(name: &str, left: &IrExpression, right: &IrExpression) -> TokenStream {
    let func = format_ident!("{}", name);
    let left_tokens = left.codegen();
    let right_tokens = right.codegen();
    quote! {{
        let left_tmp = (#left_tokens).clone();
        let right_tmp = (#right_tokens).clone();
        runtime::value::ops::#func(left_tmp, right_tmp)
    }}
}

pub(super) fn equality_op(name: &str, left: &IrExpression, right: &IrExpression) -> TokenStream {
    let func = format_ident!("{}", name);
    let left_tokens = left.codegen();
    let right_tokens = right.codegen();
    quote! {{
        let left_tmp = runtime::value::types::into_value(#left_tokens.clone());
        let right_tmp = runtime::value::types::into_value(#right_tokens.clone());
        runtime::value::ops::#func(&left_tmp, &right_tmp)
    }}
}

fn get_property_tokens(target: &IrExpression, property: &str) -> TokenStream {
    let target_tokens = target.codegen();
    let property_literal = Literal::string(property);
    quote! {{
        let target_tmp = runtime::value::into_value((#target_tokens).clone());
        runtime::value::ops::get_property(target_tmp, #property_literal)
    }}
}

fn get_property_dynamic_tokens(target: &IrExpression, property: &IrExpression) -> TokenStream {
    let target_tokens = target.codegen();
    let property_tokens = property.codegen();
    quote! {{
        let target_tmp = runtime::value::into_value((#target_tokens).clone());
        let property_tmp = runtime::value::into_value((#property_tokens).clone());
        runtime::value::ops::get_property_value(target_tmp, property_tmp)
    }}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Codegen;
    use ir::{IrExpression, IrLiteral, ValueCall};
    use proc_macro2::TokenStream;
    use quote::{format_ident, quote};

    fn ident(name: &str) -> IrExpression {
        IrExpression::Identifier(name.into())
    }

    fn number(value: f64) -> IrExpression {
        IrExpression::Literal(IrLiteral::Number(value))
    }

    fn boxed(expr: &IrExpression) -> Box<IrExpression> {
        Box::new(expr.clone())
    }

    fn assert_tokens_eq(actual: TokenStream, expected: TokenStream) {
        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn coerce_tokens_wraps_expression_codegen() {
        let expr = number(5.0);
        let expected_inner = expr.codegen();

        assert_tokens_eq(
            coerce_tokens(&expr),
            quote! { runtime::value::into_value(#expected_inner) },
        );
    }

    #[test]
    fn binary_value_generators_clone_inputs_and_delegate() {
        let left = ident("lhs");
        let right = number(2.0);

        assert_binary_value_tokens("div", div_tokens, &left, &right);
        assert_binary_value_tokens("mul", mul_tokens, &left, &right);
        assert_binary_value_tokens("modulo", mod_tokens, &left, &right);
    }

    #[test]
    fn binary_bool_generators_clone_inputs_and_delegate() {
        let left = number(1.0);
        let right = ident("rhs");

        assert_binary_bool_tokens("greater_than", greater_than_tokens, &left, &right);
        assert_binary_bool_tokens(
            "greater_than_or_equal",
            greater_than_or_equal_tokens,
            &left,
            &right,
        );
        assert_binary_bool_tokens("less_than", less_than_tokens, &left, &right);
        assert_binary_bool_tokens(
            "less_than_or_equal",
            less_than_or_equal_tokens,
            &left,
            &right,
        );
    }

    #[test]
    fn equality_generators_convert_inputs_to_values() {
        let left = ident("lhs");
        let right = number(0.0);

        assert_equality_tokens("loose_equal_refs", equal_tokens, &left, &right);
        assert_equality_tokens("strict_equal_refs", strict_equal_tokens, &left, &right);
        assert_equality_tokens("loose_not_equal_refs", not_equal_tokens, &left, &right);
        assert_equality_tokens(
            "strict_not_equal_refs",
            strict_not_equal_tokens,
            &left,
            &right,
        );
    }

    #[test]
    fn property_tokens_convert_targets_and_properties() {
        let target = ident("object");
        let property_expr = ident("prop");

        let target_tokens = target.codegen();
        let property_tokens = property_expr.codegen();

        assert_tokens_eq(
            get_property_tokens(&target, "field"),
            quote! {{
                let target_tmp = runtime::value::into_value((#target_tokens).clone());
                runtime::value::ops::get_property(target_tmp, "field")
            }},
        );

        assert_tokens_eq(
            get_property_dynamic_tokens(&target, &property_expr),
            quote! {{
                let target_tmp = runtime::value::into_value((#target_tokens).clone());
                let property_tmp = runtime::value::into_value((#property_tokens).clone());
                runtime::value::ops::get_property_value(target_tmp, property_tmp)
            }},
        );
    }

    #[test]
    fn value_call_tokens_dispatches_all_variants() {
        let left = ident("left");
        let right = number(3.0);
        let value_expr = ident("value");
        let target = ident("obj");
        let dynamic_prop = number(1.0);

        let cases: Vec<(ValueCall, TokenStream)> = vec![
            (ValueCall::Coerce { expr: boxed(&value_expr) }, coerce_tokens(&value_expr)),
            (ValueCall::Add { left: boxed(&left), right: boxed(&right) }, add_tokens(&left, &right)),
            (ValueCall::Sub { left: boxed(&left), right: boxed(&right) }, sub_tokens(&left, &right)),
            (ValueCall::Mul { left: boxed(&left), right: boxed(&right) }, mul_tokens(&left, &right)),
            (ValueCall::Div { left: boxed(&left), right: boxed(&right) }, div_tokens(&left, &right)),
            (ValueCall::Mod { left: boxed(&left), right: boxed(&right) }, mod_tokens(&left, &right)),
            (
                ValueCall::Equal {
                    left: boxed(&left),
                    right: boxed(&right),
                },
                equal_tokens(&left, &right),
            ),
            (
                ValueCall::StrictEqual {
                    left: boxed(&left),
                    right: boxed(&right),
                },
                strict_equal_tokens(&left, &right),
            ),
            (
                ValueCall::NotEqual {
                    left: boxed(&left),
                    right: boxed(&right),
                },
                not_equal_tokens(&left, &right),
            ),
            (
                ValueCall::StrictNotEqual {
                    left: boxed(&left),
                    right: boxed(&right),
                },
                strict_not_equal_tokens(&left, &right),
            ),
            (
                ValueCall::LessThan {
                    left: boxed(&left),
                    right: boxed(&right),
                },
                less_than_tokens(&left, &right),
            ),
            (
                ValueCall::LessThanOrEqual {
                    left: boxed(&left),
                    right: boxed(&right),
                },
                less_than_or_equal_tokens(&left, &right),
            ),
            (
                ValueCall::GreaterThan {
                    left: boxed(&left),
                    right: boxed(&right),
                },
                greater_than_tokens(&left, &right),
            ),
            (
                ValueCall::GreaterThanOrEqual {
                    left: boxed(&left),
                    right: boxed(&right),
                },
                greater_than_or_equal_tokens(&left, &right),
            ),
            (
                ValueCall::LogicalNot {
                    expr: boxed(&value_expr),
                },
                logical_not_tokens(&value_expr),
            ),
            (
                ValueCall::GetProperty {
                    target: boxed(&target),
                    property: "field".into(),
                },
                get_property_tokens(&target, "field"),
            ),
            (
                ValueCall::GetPropertyDynamic {
                    target: boxed(&target),
                    property: boxed(&dynamic_prop),
                },
                get_property_dynamic_tokens(&target, &dynamic_prop),
            ),
        ];

        for (call, expected) in cases {
            assert_tokens_eq(value_call_tokens(&call), expected);
        }
    }

    fn assert_binary_value_tokens<F>(
        func_name: &str,
        generator: F,
        left: &IrExpression,
        right: &IrExpression,
    ) where
        F: Fn(&IrExpression, &IrExpression) -> TokenStream,
    {
        let left_tokens = left.codegen();
        let right_tokens = right.codegen();
        let func = format_ident!("{}", func_name);

        assert_tokens_eq(
            generator(left, right),
            quote! {{
                let left_tmp = (#left_tokens).clone();
                let right_tmp = (#right_tokens).clone();
                runtime::value::ops::#func(left_tmp, right_tmp)
            }},
        );
    }

    fn assert_binary_bool_tokens<F>(
        func_name: &str,
        generator: F,
        left: &IrExpression,
        right: &IrExpression,
    ) where
        F: Fn(&IrExpression, &IrExpression) -> TokenStream,
    {
        let left_tokens = left.codegen();
        let right_tokens = right.codegen();
        let func = format_ident!("{}", func_name);

        assert_tokens_eq(
            generator(left, right),
            quote! {{
                let left_tmp = (#left_tokens).clone();
                let right_tmp = (#right_tokens).clone();
                runtime::value::ops::#func(left_tmp, right_tmp)
            }},
        );
    }

    fn assert_equality_tokens<F>(
        func_name: &str,
        generator: F,
        left: &IrExpression,
        right: &IrExpression,
    ) where
        F: Fn(&IrExpression, &IrExpression) -> TokenStream,
    {
        let left_tokens = left.codegen();
        let right_tokens = right.codegen();
        let func = format_ident!("{}", func_name);

        assert_tokens_eq(
            generator(left, right),
            quote! {{
                let left_tmp = runtime::value::types::into_value(#left_tokens.clone());
                let right_tmp = runtime::value::types::into_value(#right_tokens.clone());
                runtime::value::ops::#func(&left_tmp, &right_tmp)
            }},
        );
    }
}
