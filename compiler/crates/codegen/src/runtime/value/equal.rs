use ir::IrExpression;
use proc_macro2::TokenStream;

pub(crate) fn equal_tokens(left: &IrExpression, right: &IrExpression) -> TokenStream {
    super::equality_op("loose_equal_refs", left, right)
}
