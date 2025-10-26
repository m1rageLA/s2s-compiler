use ir::IrExpression;
use proc_macro2::TokenStream;

pub(crate) fn greater_than_tokens(left: &IrExpression, right: &IrExpression) -> TokenStream {
    super::binary_bool_op("greater_than", left, right)
}
