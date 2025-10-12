use ir::IrExpression;
use proc_macro2::TokenStream;
use quote::quote;

use crate::Codegen;

pub(crate) fn conditional_tokens(
    test: &IrExpression,
    consequent: &IrExpression,
    alternate: &IrExpression,
) -> TokenStream {
    let test_tokens = test.codegen();
    let consequent_tokens = consequent.codegen();
    let alternate_tokens = alternate.codegen();
    quote! { if #test_tokens { #consequent_tokens } else { #alternate_tokens } }
}
