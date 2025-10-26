use ir::IrExpression;
use proc_macro2::TokenStream;

pub(crate) fn mod_tokens(left: &IrExpression, right: &IrExpression) -> TokenStream {
    super::number_op("mod_number", left, right)
}
