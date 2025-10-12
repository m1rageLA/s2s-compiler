use ir::{
    ConsoleCall, IrArrowBody, IrBinOp, IrExpression, IrLiteral, IrParam, IrTemplatePart,
    RuntimeNamespace,
};
use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};

use crate::Codegen;
use crate::function::render_type;

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
            IrExpression::Template(parts) => template_literal_tokens(parts),

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

            IrExpression::Call { callee, args } => call_tokens(callee, args),
            IrExpression::Array(_) => unsupported_expr("array expression"),
            IrExpression::Member { object, property } => member_tokens(object, property),
            IrExpression::SuperCall { .. } => unsupported_expr("super call"),
            IrExpression::Arrow { params, body } => arrow_tokens(params, body),
        }
    }
}

impl Codegen for IrLiteral {
    type Output = TokenStream;
    fn codegen(&self) -> TokenStream {
        match self {
            IrLiteral::Number(value) => {
                let lit = Literal::f64_unsuffixed(*value);
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

fn arrow_tokens(params: &[IrParam], body: &IrArrowBody) -> TokenStream {
    let param_bindings: Vec<TokenStream> = params
        .iter()
        .map(|param| {
            let ident = format_ident!("{}", param.name);
            let ty = render_type(&param.ty);
            quote! { #ident: #ty }
        })
        .collect();

    match body {
        IrArrowBody::Expr(expr) => {
            let params = &param_bindings;
            let expr_tokens = expr.codegen();
            quote! { move | #( #params ),* | { #expr_tokens } }
        }
        IrArrowBody::Block(stmts) => {
            let params = &param_bindings;
            let stmt_tokens = stmts.iter().map(|stmt| stmt.codegen());
            quote! { move | #( #params ),* | { #( #stmt_tokens )* } }
        }
    }
}
fn template_literal_tokens(parts: &[IrTemplatePart]) -> TokenStream {
    let mut format_string = String::new();
    let mut expr_tokens: Vec<TokenStream> = Vec::new();

    for part in parts {
        match part {
            IrTemplatePart::String(text) => format_string.push_str(&escape_format_text(text)),
            IrTemplatePart::Expr(expr) => {
                format_string.push_str("{}");
                let inner = expr.codegen();
                expr_tokens.push(quote! { runtime::console::stringify(&(#inner)) });
            }
        }
    }

    let fmt_literal = Literal::string(&format_string);

    if expr_tokens.is_empty() {
        quote! { #fmt_literal.to_string() }
    } else {
        quote! { format!(#fmt_literal #(, #expr_tokens)*) }
    }
}

fn call_tokens(callee: &IrExpression, args: &[IrExpression]) -> TokenStream {
    let callee_tokens = callee.codegen();
    let arg_tokens: Vec<TokenStream> = args.iter().map(|arg| arg.codegen()).collect();
    quote! { (#callee_tokens)( #( #arg_tokens ),* ) }
}

fn member_tokens(object: &IrExpression, property: &str) -> TokenStream {
    let object_tokens = object.codegen();
    let property_ident = format_ident!("{}", property);
    quote! { (#object_tokens).#property_ident }
}

fn escape_format_text(text: &str) -> String {
    let mut escaped = String::with_capacity(text.len());
    for ch in text.chars() {
        match ch {
            '{' => escaped.push_str("{{"),
            '}' => escaped.push_str("}}"),
            _ => escaped.push(ch),
        }
    }
    escaped
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
