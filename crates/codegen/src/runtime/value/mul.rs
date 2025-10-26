use ir::IrExpression;
use proc_macro2::TokenStream;

pub(crate) fn mul_tokens(left: &IrExpression, right: &IrExpression) -> TokenStream {
    super::number_op("mul_number", left, right)
}
