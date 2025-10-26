use ir::{IrExpression, ValueCall};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::Codegen;

pub(crate) fn value_call_tokens(call: &ValueCall) -> TokenStream {
    match call {
        ValueCall::Coerce { expr } => {
            let expr_tokens = expr.codegen();
            quote! { runtime::value::into_value(#expr_tokens) }
        }
        ValueCall::Add { left, right } => binary_value_op("add", left, right),
        ValueCall::Sub { left, right } => number_op("sub_number", left, right),
        ValueCall::Mul { left, right } => number_op("mul_number", left, right),
        ValueCall::Div { left, right } => number_op("div_number", left, right),
        ValueCall::Mod { left, right } => number_op("mod_number", left, right),
        ValueCall::Equal { left, right } => equality_op("loose_equal_refs", left, right),
        ValueCall::StrictEqual { left, right } => equality_op("strict_equal_refs", left, right),
        ValueCall::NotEqual { left, right } => equality_op("loose_not_equal_refs", left, right),
        ValueCall::StrictNotEqual { left, right } => {
            equality_op("strict_not_equal_refs", left, right)
        }
        ValueCall::LessThan { left, right } => binary_bool_op("less_than", left, right),
        ValueCall::LessThanOrEqual { left, right } => {
            binary_bool_op("less_than_or_equal", left, right)
        }
        ValueCall::GreaterThan { left, right } => binary_bool_op("greater_than", left, right),
        ValueCall::GreaterThanOrEqual { left, right } => {
            binary_bool_op("greater_than_or_equal", left, right)
        }
    }
}

fn binary_value_op(name: &str, left: &IrExpression, right: &IrExpression) -> TokenStream {
    let func = format_ident!("{}", name);
    let left_tokens = left.codegen();
    let right_tokens = right.codegen();
    quote! { runtime::value::ops::#func(#left_tokens, #right_tokens) }
}

fn number_op(name: &str, left: &IrExpression, right: &IrExpression) -> TokenStream {
    let func = format_ident!("{}", name);
    let left_tokens = left.codegen();
    let right_tokens = right.codegen();
    quote! { runtime::value::ops::#func(#left_tokens, #right_tokens) }
}

fn binary_bool_op(name: &str, left: &IrExpression, right: &IrExpression) -> TokenStream {
    let func = format_ident!("{}", name);
    let left_tokens = left.codegen();
    let right_tokens = right.codegen();
    quote! { runtime::value::ops::#func(#left_tokens, #right_tokens) }
}

fn equality_op(name: &str, left: &IrExpression, right: &IrExpression) -> TokenStream {
    let func = format_ident!("{}", name);
    let left_tokens = left.codegen();
    let right_tokens = right.codegen();
    quote! {{
        let left_tmp = runtime::value::types::into_value(#left_tokens.clone());
        let right_tmp = runtime::value::types::into_value(#right_tokens.clone());
        runtime::value::ops::#func(&left_tmp, &right_tmp)
    }}
}
