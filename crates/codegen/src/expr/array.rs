use crate::Codegen;
use ir::IrExpression;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn array_tokens(items: &[IrExpression]) -> TokenStream {
    let item_tokens: Vec<TokenStream> = items.iter().map(|item| item.codegen()).collect();
    quote! { [ #( #item_tokens ),* ] }
}

#[test]
fn test_array_tokens() {
    let items = vec![
        IrExpression::Literal(ir::IrLiteral::Number(1.0)),
        IrExpression::Literal(ir::IrLiteral::Number(2.0)),
        IrExpression::Literal(ir::IrLiteral::Number(3.0)),
    ];
    let tokens = array_tokens(&items);

    assert_eq!(tokens.to_string(), "[1.0 , 2.0 , 3.0]");
}
