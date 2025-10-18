use ir::IrExpression;
use proc_macro2::TokenStream;
use quote::quote;

use crate::Codegen;

pub(crate) fn call_tokens(callee: &IrExpression, args: &[IrExpression]) -> TokenStream {
    let callee_tokens = callee.codegen();
    let arg_tokens: Vec<TokenStream> = args.iter().map(|arg| arg.codegen()).collect();
    quote! { (#callee_tokens)( #( #arg_tokens ),* ) }
}
