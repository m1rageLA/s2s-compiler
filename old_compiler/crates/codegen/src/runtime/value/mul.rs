use ir::IrExpression;
use proc_macro2::TokenStream;

pub(crate) fn mul_tokens(left: &IrExpression, right: &IrExpression) -> TokenStream {
    super::binary_value_op("mul", left, right)
}
