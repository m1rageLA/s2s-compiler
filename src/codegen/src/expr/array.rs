use proc_macro2::TokenStream;
use quote::quote;
use ir::IrExpression;
use crate::Codegen;    

pub(crate) fn array_tokens(items: &[IrExpression]) -> TokenStream {
    let item_tokens: Vec<TokenStream> = items.iter().map(|item| item.codegen()).collect();
    quote! { [ #( #item_tokens ),* ] }
}      