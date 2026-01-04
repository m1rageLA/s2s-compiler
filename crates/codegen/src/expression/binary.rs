use ir::{IrBinOp, IrType};
use proc_macro2::TokenStream;
use quote::quote;

use super::unsupported::unsupported_bin_op;
use crate::{typing, Codegen};

pub(crate) fn binary_op_tokens(op: IrBinOp, left: &ir::IrExpression, right: &ir::IrExpression) -> TokenStream {
    let left_tokens = left.codegen();
    let right_tokens = right.codegen();

    let left_ty = typing::infer_expression_type(left);
    let right_ty = typing::infer_expression_type(right);

    let dynamic_any = matches!(left_ty, Some(IrType::Any | IrType::Value))
        || matches!(right_ty, Some(IrType::Any | IrType::Value));

    match op {
        IrBinOp::Add => {
            let dynamic = dynamic_any || left_ty.is_none() || right_ty.is_none();
            if dynamic {
                quote! { runtime::value::ops::add(#left_tokens, #right_tokens) }
            } else if matches!(left_ty, Some(IrType::Str)) || matches!(right_ty, Some(IrType::Str)) {
                quote! { format!("{}{}", #left_tokens, #right_tokens) }
            } else if matches!(left_ty, Some(IrType::Number)) && matches!(right_ty, Some(IrType::Number)) {
                quote! { (#left_tokens) + (#right_tokens) }
            } else {
                quote! { runtime::value::ops::add(#left_tokens, #right_tokens) }
            }
        }
        IrBinOp::Sub => {
            let dynamic = dynamic_any
                || !matches!(left_ty, Some(IrType::Number))
                || !matches!(right_ty, Some(IrType::Number));
            if dynamic {
                quote! { runtime::value::ops::sub(#left_tokens, #right_tokens) }
            } else {
                quote! { (#left_tokens) - (#right_tokens) }
            }
        }
        IrBinOp::Mul => {
            let dynamic = dynamic_any
                || !matches!(left_ty, Some(IrType::Number))
                || !matches!(right_ty, Some(IrType::Number));
            if dynamic {
                quote! { runtime::value::ops::mul(#left_tokens, #right_tokens) }
            } else {
                quote! { (#left_tokens) * (#right_tokens) }
            }
        }
        IrBinOp::Div => {
            let dynamic = dynamic_any
                || !matches!(left_ty, Some(IrType::Number))
                || !matches!(right_ty, Some(IrType::Number));
            if dynamic {
                quote! { runtime::value::ops::div(#left_tokens, #right_tokens) }
            } else {
                quote! { (#left_tokens) / (#right_tokens) }
            }
        }
        IrBinOp::Mod => {
            let dynamic = dynamic_any
                || !matches!(left_ty, Some(IrType::Number))
                || !matches!(right_ty, Some(IrType::Number));
            if dynamic {
                quote! { runtime::value::ops::modulo(#left_tokens, #right_tokens) }
            } else {
                quote! { (#left_tokens) % (#right_tokens) }
            }
        }
        IrBinOp::Equal | IrBinOp::StrictEqual => {
            let dynamic = dynamic_any
                || left_ty.is_none()
                || right_ty.is_none()
                || left_ty != right_ty;
            if dynamic {
                if matches!(op, IrBinOp::StrictEqual) {
                    quote! { runtime::value::ops::strict_equal(#left_tokens, #right_tokens) }
                } else {
                    quote! { runtime::value::ops::loose_equal(#left_tokens, #right_tokens) }
                }
            } else {
                quote! { (#left_tokens) == (#right_tokens) }
            }
        }
        IrBinOp::NotEqual | IrBinOp::StrictNotEqual => {
            let dynamic = dynamic_any
                || left_ty.is_none()
                || right_ty.is_none()
                || left_ty != right_ty;
            if dynamic {
                if matches!(op, IrBinOp::StrictNotEqual) {
                    quote! { runtime::value::ops::strict_not_equal(#left_tokens, #right_tokens) }
                } else {
                    quote! { runtime::value::ops::loose_not_equal(#left_tokens, #right_tokens) }
                }
            } else {
                quote! { (#left_tokens) != (#right_tokens) }
            }
        }
        IrBinOp::LessThan => {
            let dynamic = dynamic_any
                || !matches!(left_ty, Some(IrType::Number))
                || !matches!(right_ty, Some(IrType::Number));
            if dynamic {
                quote! { runtime::value::ops::less_than(#left_tokens, #right_tokens) }
            } else {
                quote! { (#left_tokens) < (#right_tokens) }
            }
        }
        IrBinOp::LessThanOrEqual => {
            let dynamic = dynamic_any
                || !matches!(left_ty, Some(IrType::Number))
                || !matches!(right_ty, Some(IrType::Number));
            if dynamic {
                quote! { runtime::value::ops::less_than_or_equal(#left_tokens, #right_tokens) }
            } else {
                quote! { (#left_tokens) <= (#right_tokens) }
            }
        }
        IrBinOp::GreaterThan => {
            let dynamic = dynamic_any
                || !matches!(left_ty, Some(IrType::Number))
                || !matches!(right_ty, Some(IrType::Number));
            if dynamic {
                quote! { runtime::value::ops::greater_than(#left_tokens, #right_tokens) }
            } else {
                quote! { (#left_tokens) > (#right_tokens) }
            }
        }
        IrBinOp::GreaterThanOrEqual => {
            let dynamic = dynamic_any
                || !matches!(left_ty, Some(IrType::Number))
                || !matches!(right_ty, Some(IrType::Number));
            if dynamic {
                quote! { runtime::value::ops::greater_than_or_equal(#left_tokens, #right_tokens) }
            } else {
                quote! { (#left_tokens) >= (#right_tokens) }
            }
        }
        IrBinOp::LeftShift => {
            let dynamic = dynamic_any
                || !matches!(left_ty, Some(IrType::Number))
                || !matches!(right_ty, Some(IrType::Number));
            if dynamic {
                quote! {
                    ((runtime::value::into_value(#left_tokens).into_number() as i64)
                        << (runtime::value::into_value(#right_tokens).into_number() as i64)) as f64
                }
            } else {
                quote! { ((#left_tokens as i64) << (#right_tokens as i64)) as f64 }
            }
        }
        IrBinOp::RightShift => {
            let dynamic = dynamic_any
                || !matches!(left_ty, Some(IrType::Number))
                || !matches!(right_ty, Some(IrType::Number));
            if dynamic {
                quote! {
                    ((runtime::value::into_value(#left_tokens).into_number() as i64)
                        >> (runtime::value::into_value(#right_tokens).into_number() as i64)) as f64
                }
            } else {
                quote! { ((#left_tokens as i64) >> (#right_tokens as i64)) as f64 }
            }
        }
        IrBinOp::BitwiseOr => {
            let dynamic = dynamic_any
                || !matches!(left_ty, Some(IrType::Number))
                || !matches!(right_ty, Some(IrType::Number));
            if dynamic {
                quote! {
                    ((runtime::value::into_value(#left_tokens).into_number() as i64)
                        | (runtime::value::into_value(#right_tokens).into_number() as i64)) as f64
                }
            } else {
                quote! { ((#left_tokens as i64) | (#right_tokens as i64)) as f64 }
            }
        }
        IrBinOp::BitwiseXor => {
            let dynamic = dynamic_any
                || !matches!(left_ty, Some(IrType::Number))
                || !matches!(right_ty, Some(IrType::Number));
            if dynamic {
                quote! {
                    ((runtime::value::into_value(#left_tokens).into_number() as i64)
                        ^ (runtime::value::into_value(#right_tokens).into_number() as i64)) as f64
                }
            } else {
                quote! { ((#left_tokens as i64) ^ (#right_tokens as i64)) as f64 }
            }
        }
        IrBinOp::BitwiseAnd => {
            let dynamic = dynamic_any
                || !matches!(left_ty, Some(IrType::Number))
                || !matches!(right_ty, Some(IrType::Number));
            if dynamic {
                quote! {
                    ((runtime::value::into_value(#left_tokens).into_number() as i64)
                        & (runtime::value::into_value(#right_tokens).into_number() as i64)) as f64
                }
            } else {
                quote! { ((#left_tokens as i64) & (#right_tokens as i64)) as f64 }
            }
        }
        IrBinOp::LogicalOr => {
            let dynamic = dynamic_any
                || !matches!(left_ty, Some(IrType::Bool))
                || !matches!(right_ty, Some(IrType::Bool));
            if dynamic {
                quote! {{
                    let lhs_bool = runtime::value::into_value((#left_tokens).clone()).to_boolean();
                    let rhs_bool = runtime::value::into_value((#right_tokens).clone()).to_boolean();
                    lhs_bool || rhs_bool
                }}
            } else {
                quote! { (#left_tokens) || (#right_tokens) }
            }
        }
        IrBinOp::LogicalAnd => {
            let dynamic = dynamic_any
                || !matches!(left_ty, Some(IrType::Bool))
                || !matches!(right_ty, Some(IrType::Bool));
            if dynamic {
                quote! {{
                    let lhs_bool = runtime::value::into_value((#left_tokens).clone()).to_boolean();
                    let rhs_bool = runtime::value::into_value((#right_tokens).clone()).to_boolean();
                    lhs_bool && rhs_bool
                }}
            } else {
                quote! { (#left_tokens) && (#right_tokens) }
            }
        }
        IrBinOp::UnsignedRightShift => unsupported_bin_op("unsigned right shift"),
        IrBinOp::In => unsupported_bin_op("in"),
        IrBinOp::InstanceOf => unsupported_bin_op("instanceof"),
        IrBinOp::Exp => {
            let dynamic = dynamic_any
                || !matches!(left_ty, Some(IrType::Number))
                || !matches!(right_ty, Some(IrType::Number));
            if dynamic {
                unsupported_bin_op("exponentiation")
            } else {
                quote! { (#left_tokens).powf(#right_tokens) }
            }
        }
        IrBinOp::Unsupported => unsupported_bin_op("unsupported"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;
    use ir::IrExpression;
    use syn::{Expr, parse2};

    fn norm(ts: &TokenStream) -> String {
        ts.to_string().replace([' ', '\n', '\t'], "")
    }

    fn assert_parses(expr: &TokenStream) {
        parse2::<Expr>(expr.clone()).expect("generated tokens must be a valid Rust expression");
    }

    #[test]
    fn numeric_ops_use_native_arithmetic_when_types_are_known() {
        typing::reset();
        typing::define("lhs", IrType::Number);
        typing::define("rhs", IrType::Number);

        let left = IrExpression::Identifier("lhs".into());
        let right = IrExpression::Identifier("rhs".into());

        let add = binary_op_tokens(IrBinOp::Add, &left, &right);
        assert_eq!(norm(&add), norm(&quote! { (lhs) + (rhs) }));

        let mul = binary_op_tokens(IrBinOp::Mul, &left, &right);
        assert_eq!(norm(&mul), norm(&quote! { (lhs) * (rhs) }));

        let lt = binary_op_tokens(IrBinOp::LessThan, &left, &right);
        assert_eq!(norm(&lt), norm(&quote! { (lhs) < (rhs) }));
    }

    #[test]
    fn string_addition_falls_back_to_formatting() {
        typing::reset();
        typing::define("a", IrType::Str);
        typing::define("b", IrType::Str);

        let left = IrExpression::Identifier("a".into());
        let right = IrExpression::Identifier("b".into());

        let tokens = binary_op_tokens(IrBinOp::Add, &left, &right);
        assert!(tokens.to_string().contains("format"));
        assert_parses(&tokens);
    }

    #[test]
    fn unknown_types_use_value_runtime() {
        typing::reset();
        let left = IrExpression::Identifier("lhs".into());
        let right = IrExpression::Identifier("rhs".into());

        let tokens = binary_op_tokens(IrBinOp::Add, &left, &right);
        assert!(tokens.to_string().contains("runtime :: value :: ops :: add"));
    }
}
