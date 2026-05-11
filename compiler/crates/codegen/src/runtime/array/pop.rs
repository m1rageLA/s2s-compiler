use crate::{Codegen, typing};
use ir::{IrArrayKind, IrExpression, IrType};
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn pop_tokens(target: &Box<IrExpression>, args: &Vec<IrExpression>) -> TokenStream {
    let target_ty = typing::infer_expression_type(target.as_ref());
    let target_tokens = target.codegen();

    match target_ty {
        Some(IrType::Array(IrArrayKind::Number))
        | Some(IrType::Array(IrArrayKind::Bool))
        | Some(IrType::Array(IrArrayKind::Object(_))) => quote! {{
            let ts_2_rs_target = &mut #target_tokens;
            ts_2_rs_target.pop().unwrap_or_default()
        }},
        Some(IrType::Array(IrArrayKind::Str)) => quote! {{
            let ts_2_rs_target = &mut #target_tokens;
            ts_2_rs_target.pop().unwrap_or_default()
        }},
        _ => {
            let args_tokens: TokenStream = args
                .iter()
                .map(|arg| arg.codegen())
                .collect::<Vec<_>>()
                .into_iter()
                .collect();
            quote! { runtime::array::pop_number(&mut #target_tokens, #args_tokens) }
        }
    }
}
