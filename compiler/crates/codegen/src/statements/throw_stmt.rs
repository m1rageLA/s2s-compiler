use ir::IrExpression;
use proc_macro2::TokenStream;
use quote::quote;

use crate::Codegen;

pub(crate) fn throw_tokens(expr: &IrExpression) -> TokenStream {
    let value = expr.codegen();
    quote! {
        std::panic::panic_any(runtime::value::into_value((#value).clone()));
    }
}
