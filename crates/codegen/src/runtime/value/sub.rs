use ir::IrExpression;
use proc_macro2::TokenStream;

pub(crate) fn sub_tokens(left: &IrExpression, right: &IrExpression) -> TokenStream {
    super::number_op("sub_number", left, right)
}
