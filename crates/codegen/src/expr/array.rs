use crate::Codegen;
use ir::IrExpression;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn array_tokens(items: &[IrExpression]) -> TokenStream {
    let item_tokens: Vec<TokenStream> = items.iter().map(|item| item.codegen()).collect();
    quote! { [ #( #item_tokens ),* ] }
}
