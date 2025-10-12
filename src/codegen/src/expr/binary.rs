use ir::IrBinOp;
use proc_macro2::TokenStream;
use quote::quote;

use super::unsupported::unsupported_bin_op;

pub(crate) fn binary_op_tokens(
    op: IrBinOp,
    left: TokenStream,
    right: TokenStream,
) -> TokenStream {
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
