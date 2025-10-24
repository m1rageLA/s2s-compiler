use ir::{RuntimeNamespace, ValueCall};

use super::IrExpression;

pub(crate) fn coerce_to_value(expr: IrExpression) -> IrExpression {
    IrExpression::RuntimeCall(RuntimeNamespace::Value(ValueCall::Coerce {
        expr: Box::new(expr),
    }))
}

pub(crate) fn value_binary(call: ValueCall) -> IrExpression {
    IrExpression::RuntimeCall(RuntimeNamespace::Value(call))
}
