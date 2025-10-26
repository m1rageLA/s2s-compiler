use ir::IrExpression;
use proc_macro2::TokenStream;

pub(crate) fn greater_than_or_equal_tokens(
    left: &IrExpression,
    right: &IrExpression,
) -> TokenStream {
    super::binary_bool_op("greater_than_or_equal", left, right)
}
