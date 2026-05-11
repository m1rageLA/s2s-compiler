use ir::IrExpression;
use proc_macro2::TokenStream;

pub(crate) fn sub_tokens(left: &IrExpression, right: &IrExpression) -> TokenStream {
    super::binary_value_op("sub", left, right)
}
