use ir::IrExpression;
use proc_macro2::TokenStream;

pub(crate) fn less_than_tokens(left: &IrExpression, right: &IrExpression) -> TokenStream {
    super::binary_bool_op("less_than", left, right)
}
