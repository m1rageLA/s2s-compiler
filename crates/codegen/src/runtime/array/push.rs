use crate::{Codegen, typing};
use ir::{IrArrayKind, IrExpression, IrType};
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn push_tokens(target: &IrExpression, args: &[IrExpression]) -> TokenStream {
    let target_ty = typing::infer_expression_type(target);
    let target_tokens = target.codegen();

    match target_ty {
        Some(IrType::Array(kind)) => match kind {
            IrArrayKind::Number | IrArrayKind::Str | IrArrayKind::Bool => {
                typed_push(&target_tokens, args, kind)
            }
            _ => runtime_push(&target_tokens, args),
        },
        _ => runtime_push(&target_tokens, args),
    }
}

fn typed_push(target_tokens: &TokenStream, args: &[IrExpression], kind: IrArrayKind) -> TokenStream {
    let value_tokens: Vec<TokenStream> = args
        .iter()
        .map(|arg| {
            let tokens = arg.codegen();
            let ty = typing::infer_expression_type(arg);
            match kind {
                IrArrayKind::Number => typing::coerce_to_type(tokens, &IrType::Number, ty),
                IrArrayKind::Str => typing::coerce_to_type(tokens, &IrType::Str, ty),
                IrArrayKind::Bool => typing::coerce_to_type(tokens, &IrType::Bool, ty),
                _ => tokens,
            }
        })
        .collect();
    quote! {{
        let ts_2_rs_target = &mut #target_tokens;
        #( ts_2_rs_target.push(#value_tokens); )*
        ts_2_rs_target.len()
    }}
}

fn runtime_push(target_tokens: &TokenStream, args: &[IrExpression]) -> TokenStream {
    let value_tokens: Vec<TokenStream> = args
        .iter()
        .map(|arg| {
            let expr_tokens = arg.codegen();
            quote! { runtime::value::into_value(#expr_tokens) }
        })
        .collect();
    quote! { runtime::array::push(&mut #target_tokens, vec![ #( #value_tokens ),* ]) }
}
