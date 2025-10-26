use ir::IrExpression;
use proc_macro2::TokenStream;

pub(crate) fn strict_equal_tokens(left: &IrExpression, right: &IrExpression) -> TokenStream {
    super::equality_op("strict_equal_refs", left, right)
}
