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
