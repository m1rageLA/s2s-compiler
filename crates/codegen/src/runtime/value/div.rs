use ir::IrExpression;
use proc_macro2::TokenStream;

pub(crate) fn div_tokens(left: &IrExpression, right: &IrExpression) -> TokenStream {
    super::number_op("div_number", left, right)
}
