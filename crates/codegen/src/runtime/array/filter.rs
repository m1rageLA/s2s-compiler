use crate::{Codegen, typing};
use ir::{IrArrayKind, IrExpression, IrType};
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn filter_tokens(target: &IrExpression, callback: &IrExpression) -> TokenStream {
    let target_tokens = target.codegen();
    let callback_tokens = callback.codegen();
    let element_kind = match typing::infer_expression_type(target) {
        Some(IrType::Array(kind)) => Some(kind),
        _ => None,
    };

    let param_ty = callback_param_type(callback);

    if let (Some(kind), Some(param)) = (element_kind, param_ty) {
        if matches!(kind, IrArrayKind::Number | IrArrayKind::Str | IrArrayKind::Bool) {
            let arg = argument_tokens(&param, kind);
            return quote! {{
                let ts_2_rs_cb = #callback_tokens;
                #target_tokens
                    .iter()
                    .cloned()
                    .filter(|item| {
                        let predicate = ts_2_rs_cb(#arg);
                        !runtime::value::ops::logical_not(predicate)
                    })
                    .collect::<::std::vec::Vec<_>>()
            }};
        }
    }

    quote! { runtime::array::filter(&#target_tokens, #callback_tokens) }
}

fn callback_param_type(callback: &IrExpression) -> Option<IrType> {
    match callback {
        IrExpression::Arrow { params, .. } => params.first().map(|p| p.ty),
        IrExpression::Function(func) => func.params.first().map(|p| p.ty),
        _ => None,
    }
}

fn argument_tokens(param_ty: &IrType, element_kind: IrArrayKind) -> TokenStream {
    match param_ty {
        IrType::Any | IrType::Value => quote! { runtime::value::into_value((item).clone()) },
        IrType::Number if matches!(element_kind, IrArrayKind::Number) => quote! { (item).clone() },
        IrType::Str if matches!(element_kind, IrArrayKind::Str) => quote! { (item).clone() },
        IrType::Bool if matches!(element_kind, IrArrayKind::Bool) => quote! { (item).clone() },
        _ => quote! { runtime::value::into_value((item).clone()) },
    }
}
