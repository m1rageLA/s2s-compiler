use crate::Codegen;
use ir::{IrArrayKind, IrExpression};
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn index_tokens(
    target: &IrExpression,
    index: &IrExpression,
    element: Option<IrArrayKind>,
) -> TokenStream {
    let target_tokens = target.codegen();
    let index_tokens = index.codegen();
    let _ = element; // element hint currently unused in Value mode
    quote! {{
        let index_tmp = (#index_tokens).clone();
        runtime::array::index(&#target_tokens, index_tmp)
    }}
}
