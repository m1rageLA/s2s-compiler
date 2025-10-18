use ir::IrExpression;

pub(crate) fn infer_default(_expr: &IrExpression) -> Option<ir::IrType> {
    None
}
