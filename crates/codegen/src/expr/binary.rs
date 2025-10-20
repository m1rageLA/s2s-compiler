use ir::IrBinOp;
use proc_macro2::TokenStream;
use quote::quote;

use super::unsupported::unsupported_bin_op;

pub(crate) fn binary_op_tokens(op: IrBinOp, left: TokenStream, right: TokenStream) -> TokenStream {
    match op {
        IrBinOp::Add => quote! { (#left) + (#right) },
        IrBinOp::Sub => quote! { (#left) - (#right) },
        IrBinOp::Mul => quote! { (#left) * (#right) },
        IrBinOp::Div => quote! { (#left) / (#right) },
        IrBinOp::Mod => quote! { (#left) % (#right) },
        IrBinOp::Equal | IrBinOp::StrictEqual => quote! { (#left) == (#right) },
        IrBinOp::NotEqual | IrBinOp::StrictNotEqual => quote! { (#left) != (#right) },
        IrBinOp::LessThan => quote! { (#left) < (#right) },
        IrBinOp::LessThanOrEqual => quote! { (#left) <= (#right) },
        IrBinOp::GreaterThan => quote! { (#left) > (#right) },
        IrBinOp::GreaterThanOrEqual => quote! { (#left) >= (#right) },
        IrBinOp::LeftShift => quote! { (#left) << (#right) },
        IrBinOp::RightShift => quote! { (#left) >> (#right) },
        IrBinOp::BitwiseOr => quote! { (#left) | (#right) },
        IrBinOp::BitwiseXor => quote! { (#left) ^ (#right) },
        IrBinOp::BitwiseAnd => quote! { (#left) & (#right) },
        IrBinOp::LogicalOr => quote! { (#left) || (#right) },
        IrBinOp::LogicalAnd => quote! { (#left) && (#right) },
        IrBinOp::UnsignedRightShift => unsupported_bin_op("unsigned right shift"),
        IrBinOp::In => unsupported_bin_op("in"),
        IrBinOp::InstanceOf => unsupported_bin_op("instanceof"),
        IrBinOp::Exp => unsupported_bin_op("exponentiation"),
        IrBinOp::Unsupported => unsupported_bin_op("unsupported"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    #[test]
    fn supported_binary_ops_emit_expected_tokens() {
        let left = quote!(lhs);
        let right = quote!(rhs);

        let cases = vec![
            (IrBinOp::Add, quote! { (lhs) + (rhs) }),
            (IrBinOp::Sub, quote! { (lhs) - (rhs) }),
            (IrBinOp::Mul, quote! { (lhs) * (rhs) }),
            (IrBinOp::Div, quote! { (lhs) / (rhs) }),
            (IrBinOp::Mod, quote! { (lhs) % (rhs) }),
            (IrBinOp::Equal, quote! { (lhs) == (rhs) }),
            (IrBinOp::StrictEqual, quote! { (lhs) == (rhs) }),
            (IrBinOp::NotEqual, quote! { (lhs) != (rhs) }),
            (IrBinOp::StrictNotEqual, quote! { (lhs) != (rhs) }),
            (IrBinOp::LessThan, quote! { (lhs) < (rhs) }),
            (IrBinOp::LessThanOrEqual, quote! { (lhs) <= (rhs) }),
            (IrBinOp::GreaterThan, quote! { (lhs) > (rhs) }),
            (IrBinOp::GreaterThanOrEqual, quote! { (lhs) >= (rhs) }),
            (IrBinOp::LeftShift, quote! { (lhs) << (rhs) }),
            (IrBinOp::RightShift, quote! { (lhs) >> (rhs) }),
            (IrBinOp::BitwiseOr, quote! { (lhs) | (rhs) }),
            (IrBinOp::BitwiseXor, quote! { (lhs) ^ (rhs) }),
            (IrBinOp::BitwiseAnd, quote! { (lhs) & (rhs) }),
            (IrBinOp::LogicalOr, quote! { (lhs) || (rhs) }),
            (IrBinOp::LogicalAnd, quote! { (lhs) && (rhs) }),
        ];

        for (op, expected) in cases {
            let tokens = binary_op_tokens(op, left.clone(), right.clone());
            assert_eq!(
                tokens.to_string(),
                expected.to_string(),
                "mismatch for {op:?}"
            );
        }
    }

    #[test]
    fn unsupported_binary_ops_panic_with_reason() {
        let left = quote!(lhs);
        let right = quote!(rhs);

        let cases = vec![
            (
                IrBinOp::UnsignedRightShift,
                "codegen for binary op `unsigned right shift` not implemented",
            ),
            (IrBinOp::In, "codegen for binary op `in` not implemented"),
            (
                IrBinOp::InstanceOf,
                "codegen for binary op `instanceof` not implemented",
            ),
            (
                IrBinOp::Exp,
                "codegen for binary op `exponentiation` not implemented",
            ),
            (
                IrBinOp::Unsupported,
                "codegen for binary op `unsupported` not implemented",
            ),
        ];

        for (op, message) in cases {
            let tokens = binary_op_tokens(op, left.clone(), right.clone());
            assert_eq!(
                tokens.to_string(),
                quote! { panic!(#message) }.to_string(),
                "unexpected output for {op:?}"
            );
        }
    }
}
