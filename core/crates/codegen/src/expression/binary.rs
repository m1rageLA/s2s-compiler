use ir::{IrBinOp, IrType};
use proc_macro2::TokenStream;
use quote::quote;

use super::unsupported::unsupported_bin_op;
use crate::{Codegen, typing};

pub(crate) fn binary_op_tokens(
    op: IrBinOp,
    left: &ir::IrExpression,
    right: &ir::IrExpression,
) -> TokenStream {
    let left_tokens = left.codegen();
    let right_tokens = right.codegen();

    let left_ty = typing::infer_expression_type(left);
    let right_ty = typing::infer_expression_type(right);
    let left_numeric = matches!(left_ty, Some(IrType::Number | IrType::UInt));
    let right_numeric = matches!(right_ty, Some(IrType::Number | IrType::UInt));
    let both_uint = matches!(left_ty, Some(IrType::UInt)) && matches!(right_ty, Some(IrType::UInt));

    let dynamic_any = matches!(left_ty, Some(IrType::Any | IrType::Value))
        || matches!(right_ty, Some(IrType::Any | IrType::Value));

    match op {
        IrBinOp::Add => {
            let dynamic = dynamic_any || left_ty.is_none() || right_ty.is_none();
            if dynamic {
                quote! { runtime::value::ops::add(#left_tokens, #right_tokens) }
            } else if matches!(left_ty, Some(IrType::Str)) || matches!(right_ty, Some(IrType::Str))
            {
                quote! { format!("{}{}", #left_tokens, #right_tokens) }
            } else if both_uint {
                quote! { (#left_tokens) + (#right_tokens) }
            } else if left_numeric && right_numeric {
                let left_num = if matches!(left_ty, Some(IrType::UInt)) {
                    quote! { (#left_tokens) as f64 }
                } else {
                    quote! { #left_tokens }
                };
                let right_num = if matches!(right_ty, Some(IrType::UInt)) {
                    quote! { (#right_tokens) as f64 }
                } else {
                    quote! { #right_tokens }
                };
                quote! { (#left_num) + (#right_num) }
            } else {
                quote! { runtime::value::ops::add(#left_tokens, #right_tokens) }
            }
        }
        IrBinOp::Sub => {
            let dynamic = dynamic_any || !left_numeric || !right_numeric;
            if dynamic {
                quote! { runtime::value::ops::sub(#left_tokens, #right_tokens) }
            } else {
                let left_num = if matches!(left_ty, Some(IrType::UInt)) {
                    quote! { (#left_tokens) as f64 }
                } else {
                    quote! { #left_tokens }
                };
                let right_num = if matches!(right_ty, Some(IrType::UInt)) {
                    quote! { (#right_tokens) as f64 }
                } else {
                    quote! { #right_tokens }
                };
                quote! { (#left_num) - (#right_num) }
            }
        }
        IrBinOp::Mul => {
            let dynamic = dynamic_any || !left_numeric || !right_numeric;
            if dynamic {
                quote! { runtime::value::ops::mul(#left_tokens, #right_tokens) }
            } else if both_uint {
                quote! { (#left_tokens) * (#right_tokens) }
            } else {
                let left_num = if matches!(left_ty, Some(IrType::UInt)) {
                    quote! { (#left_tokens) as f64 }
                } else {
                    quote! { #left_tokens }
                };
                let right_num = if matches!(right_ty, Some(IrType::UInt)) {
                    quote! { (#right_tokens) as f64 }
                } else {
                    quote! { #right_tokens }
                };
                quote! { (#left_num) * (#right_num) }
            }
        }
        IrBinOp::Div => {
            let dynamic = dynamic_any || !left_numeric || !right_numeric;
            if dynamic {
                quote! { runtime::value::ops::div(#left_tokens, #right_tokens) }
            } else {
                let left_num = if matches!(left_ty, Some(IrType::UInt)) {
                    quote! { (#left_tokens) as f64 }
                } else {
                    quote! { #left_tokens }
                };
                let right_num = if matches!(right_ty, Some(IrType::UInt)) {
                    quote! { (#right_tokens) as f64 }
                } else {
                    quote! { #right_tokens }
                };
                quote! { (#left_num) / (#right_num) }
            }
        }
        IrBinOp::Mod => {
            let dynamic = dynamic_any || !left_numeric || !right_numeric;
            if dynamic {
                quote! { runtime::value::ops::modulo(#left_tokens, #right_tokens) }
            } else if both_uint {
                quote! { (#left_tokens) % (#right_tokens) }
            } else {
                let left_num = if matches!(left_ty, Some(IrType::UInt)) {
                    quote! { (#left_tokens) as f64 }
                } else {
                    quote! { #left_tokens }
                };
                let right_num = if matches!(right_ty, Some(IrType::UInt)) {
                    quote! { (#right_tokens) as f64 }
                } else {
                    quote! { #right_tokens }
                };
                quote! { (#left_num) % (#right_num) }
            }
        }
        IrBinOp::Equal | IrBinOp::StrictEqual => {
            let numeric_compatible = (matches!(left_ty, Some(IrType::Number))
                && matches!(right_ty, Some(IrType::UInt)))
                || (matches!(left_ty, Some(IrType::UInt))
                    && matches!(right_ty, Some(IrType::Number)))
                || both_uint;
            let same_type = left_ty.is_some() && left_ty == right_ty;
            let dynamic = dynamic_any
                || left_ty.is_none()
                || right_ty.is_none()
                || (!same_type && !numeric_compatible);
            if dynamic {
                if matches!(op, IrBinOp::StrictEqual) {
                    quote! { runtime::value::ops::strict_equal(#left_tokens, #right_tokens) }
                } else {
                    quote! { runtime::value::ops::loose_equal(#left_tokens, #right_tokens) }
                }
            } else if both_uint {
                quote! { (#left_tokens) == (#right_tokens) }
            } else if numeric_compatible {
                let left_num = if matches!(left_ty, Some(IrType::UInt)) {
                    quote! { (#left_tokens) as f64 }
                } else {
                    quote! { #left_tokens }
                };
                let right_num = if matches!(right_ty, Some(IrType::UInt)) {
                    quote! { (#right_tokens) as f64 }
                } else {
                    quote! { #right_tokens }
                };
                quote! { (#left_num) == (#right_num) }
            } else {
                quote! { (#left_tokens) == (#right_tokens) }
            }
        }
        IrBinOp::NotEqual | IrBinOp::StrictNotEqual => {
            let numeric_compatible = (matches!(left_ty, Some(IrType::Number))
                && matches!(right_ty, Some(IrType::UInt)))
                || (matches!(left_ty, Some(IrType::UInt))
                    && matches!(right_ty, Some(IrType::Number)))
                || both_uint;
            let same_type = left_ty.is_some() && left_ty == right_ty;
            let dynamic = dynamic_any
                || left_ty.is_none()
                || right_ty.is_none()
                || (!same_type && !numeric_compatible);
            if dynamic {
                if matches!(op, IrBinOp::StrictNotEqual) {
                    quote! { runtime::value::ops::strict_not_equal(#left_tokens, #right_tokens) }
                } else {
                    quote! { runtime::value::ops::loose_not_equal(#left_tokens, #right_tokens) }
                }
            } else if both_uint {
                quote! { (#left_tokens) != (#right_tokens) }
            } else if numeric_compatible {
                let left_num = if matches!(left_ty, Some(IrType::UInt)) {
                    quote! { (#left_tokens) as f64 }
                } else {
                    quote! { #left_tokens }
                };
                let right_num = if matches!(right_ty, Some(IrType::UInt)) {
                    quote! { (#right_tokens) as f64 }
                } else {
                    quote! { #right_tokens }
                };
                quote! { (#left_num) != (#right_num) }
            } else {
                quote! { (#left_tokens) != (#right_tokens) }
            }
        }
        IrBinOp::LessThan => {
            let dynamic = dynamic_any || !left_numeric || !right_numeric;
            if dynamic {
                quote! { runtime::value::ops::less_than(#left_tokens, #right_tokens) }
            } else if both_uint {
                quote! { (#left_tokens) < (#right_tokens) }
            } else {
                let left_num = if matches!(left_ty, Some(IrType::UInt)) {
                    quote! { (#left_tokens) as f64 }
                } else {
                    quote! { #left_tokens }
                };
                let right_num = if matches!(right_ty, Some(IrType::UInt)) {
                    quote! { (#right_tokens) as f64 }
                } else {
                    quote! { #right_tokens }
                };
                quote! { (#left_num) < (#right_num) }
            }
        }
        IrBinOp::LessThanOrEqual => {
            let dynamic = dynamic_any || !left_numeric || !right_numeric;
            if dynamic {
                quote! { runtime::value::ops::less_than_or_equal(#left_tokens, #right_tokens) }
            } else if both_uint {
                quote! { (#left_tokens) <= (#right_tokens) }
            } else {
                let left_num = if matches!(left_ty, Some(IrType::UInt)) {
                    quote! { (#left_tokens) as f64 }
                } else {
                    quote! { #left_tokens }
                };
                let right_num = if matches!(right_ty, Some(IrType::UInt)) {
                    quote! { (#right_tokens) as f64 }
                } else {
                    quote! { #right_tokens }
                };
                quote! { (#left_num) <= (#right_num) }
            }
        }
        IrBinOp::GreaterThan => {
            let dynamic = dynamic_any || !left_numeric || !right_numeric;
            if dynamic {
                quote! { runtime::value::ops::greater_than(#left_tokens, #right_tokens) }
            } else if both_uint {
                quote! { (#left_tokens) > (#right_tokens) }
            } else {
                let left_num = if matches!(left_ty, Some(IrType::UInt)) {
                    quote! { (#left_tokens) as f64 }
                } else {
                    quote! { #left_tokens }
                };
                let right_num = if matches!(right_ty, Some(IrType::UInt)) {
                    quote! { (#right_tokens) as f64 }
                } else {
                    quote! { #right_tokens }
                };
                quote! { (#left_num) > (#right_num) }
            }
        }
        IrBinOp::GreaterThanOrEqual => {
            let dynamic = dynamic_any || !left_numeric || !right_numeric;
            if dynamic {
                quote! { runtime::value::ops::greater_than_or_equal(#left_tokens, #right_tokens) }
            } else if both_uint {
                quote! { (#left_tokens) >= (#right_tokens) }
            } else {
                let left_num = if matches!(left_ty, Some(IrType::UInt)) {
                    quote! { (#left_tokens) as f64 }
                } else {
                    quote! { #left_tokens }
                };
                let right_num = if matches!(right_ty, Some(IrType::UInt)) {
                    quote! { (#right_tokens) as f64 }
                } else {
                    quote! { #right_tokens }
                };
                quote! { (#left_num) >= (#right_num) }
            }
        }
        IrBinOp::LeftShift => {
            let dynamic = dynamic_any || !left_numeric || !right_numeric;
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
            let dynamic = dynamic_any || !left_numeric || !right_numeric;
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
            let dynamic = dynamic_any || !left_numeric || !right_numeric;
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
            let dynamic = dynamic_any || !left_numeric || !right_numeric;
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
            let dynamic = dynamic_any || !left_numeric || !right_numeric;
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
        IrBinOp::UnsignedRightShift => {
            let dynamic = dynamic_any || !left_numeric || !right_numeric;
            if dynamic {
                quote! {{
                    let ts_2_rs_lhs =
                        (runtime::value::into_value(#left_tokens).into_number() as i64) as u32;
                    let ts_2_rs_rhs =
                        (runtime::value::into_value(#right_tokens).into_number() as i64) as u32;
                    ((ts_2_rs_lhs >> (ts_2_rs_rhs & 31)) as f64)
                }}
            } else {
                quote! {{
                    let ts_2_rs_lhs = ((#left_tokens) as i64) as u32;
                    let ts_2_rs_rhs = ((#right_tokens) as i64) as u32;
                    ((ts_2_rs_lhs >> (ts_2_rs_rhs & 31)) as f64)
                }}
            }
        }
        IrBinOp::In => unsupported_bin_op("in"),
        IrBinOp::InstanceOf => unsupported_bin_op("instanceof"),
        IrBinOp::Exp => {
            let dynamic = dynamic_any || !left_numeric || !right_numeric;
            if dynamic {
                unsupported_bin_op("exponentiation")
            } else {
                let left_num = if matches!(left_ty, Some(IrType::UInt)) {
                    quote! { (#left_tokens) as f64 }
                } else {
                    quote! { #left_tokens }
                };
                let right_num = if matches!(right_ty, Some(IrType::UInt)) {
                    quote! { (#right_tokens) as f64 }
                } else {
                    quote! { #right_tokens }
                };
                quote! { (#left_num).powf(#right_num) }
            }
        }
        IrBinOp::Unsupported => unsupported_bin_op("unsupported"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ir::IrExpression;
    use quote::quote;
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
        assert!(
            tokens
                .to_string()
                .contains("runtime :: value :: ops :: add")
        );
    }

    #[test]
    fn uint_and_number_addition_coerces_without_runtime() {
        typing::reset();
        typing::define("lhs", IrType::UInt);
        typing::define("rhs", IrType::Number);

        let left = IrExpression::Identifier("lhs".into());
        let right = IrExpression::Identifier("rhs".into());

        let tokens = binary_op_tokens(IrBinOp::Add, &left, &right);
        let rendered = tokens.to_string();
        assert!(!rendered.contains("runtime :: value :: ops :: add"));
        assert!(rendered.contains("as f64"));
    }

    #[test]
    fn unsigned_right_shift_masks_rhs() {
        typing::reset();
        typing::define("lhs", IrType::Number);
        typing::define("rhs", IrType::Number);

        let left = IrExpression::Identifier("lhs".into());
        let right = IrExpression::Identifier("rhs".into());

        let tokens = binary_op_tokens(IrBinOp::UnsignedRightShift, &left, &right);
        let rendered = norm(&tokens);
        assert!(rendered.contains(">>"));
        assert!(rendered.contains("&31"));
        assert_parses(&tokens);
    }
}
