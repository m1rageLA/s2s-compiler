use ir::{ConsoleCall, IrBinOp, IrExpression, IrLiteral, RuntimeNamespace};
use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};

use crate::Codegen;

impl Codegen for IrExpression {
    type Output = TokenStream;

    fn codegen(&self) -> TokenStream {
        match self {
            IrExpression::Identifier(name) => {
                let ident = format_ident!("{}", name);
                quote! { #ident }
            }
            IrExpression::Literal(literal) => literal.codegen(),
            IrExpression::Binary { op, left, right } => {
                let left_tokens = left.codegen();
                let right_tokens = right.codegen();
                binary_op_tokens(*op, left_tokens, right_tokens)
            }
            IrExpression::Call { .. } => unsupported_expr("call expression"),
            IrExpression::Array(_) => unsupported_expr("array expression"),

            IrExpression::RuntimeCall(RuntimeNamespace::Console(ConsoleCall::Log(args))) => {
                let arg_tokens: Vec<TokenStream> = args
                    .iter()
                    .map(|a| {
                        let expr = a.codegen();
                        quote! { runtime::console::stringify(&(#expr)) }
                    })
                    .collect();
                quote! { runtime::console::log(vec![ #( #arg_tokens ),* ]) }
            }

            IrExpression::Member { .. } => unsupported_expr("member expression"),
            IrExpression::SuperCall { .. } => unsupported_expr("super call"),
        }
    }
}

impl Codegen for IrLiteral {
    type Output = TokenStream;

    fn codegen(&self) -> TokenStream {
        match self {
            IrLiteral::Int(value) => {
                let lit = Literal::i32_unsuffixed(*value);
                quote! { #lit }
            }
            IrLiteral::Str(value) => {
                let lit = Literal::string(value);
                quote! { #lit.to_string() }
            }
            IrLiteral::Bool(value) => {
                if *value {
                    quote! { true }
                } else {
                    quote! { false }
                }
            }
        }
    }
}

fn binary_op_tokens(op: IrBinOp, left: TokenStream, right: TokenStream) -> TokenStream {
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

fn unsupported_expr(kind: &str) -> TokenStream {
    let msg = Literal::string(&format!("codegen for {kind} not implemented"));
    quote! { panic!(#msg) }
}

fn unsupported_bin_op(name: &str) -> TokenStream {
    let msg = Literal::string(&format!("codegen for binary op `{name}` not implemented"));
    quote! { panic!(#msg) }
}
