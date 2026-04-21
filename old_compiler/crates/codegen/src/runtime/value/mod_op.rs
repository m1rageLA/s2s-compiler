use ir::IrExpression;
use proc_macro2::TokenStream;

pub(crate) fn mod_tokens(left: &IrExpression, right: &IrExpression) -> TokenStream {
    super::binary_value_op("modulo", left, right)
}
