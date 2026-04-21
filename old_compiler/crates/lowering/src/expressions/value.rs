use ir::{IrBinOp, IrExpression, RuntimeNamespace, ValueCall};

pub(crate) fn coerce_to_value(expr: IrExpression) -> IrExpression {
    match expr {
        IrExpression::RuntimeCall(RuntimeNamespace::Value(call)) => match call {
            ValueCall::Add { .. } | ValueCall::Coerce { .. } | ValueCall::GetProperty { .. } => {
                IrExpression::RuntimeCall(RuntimeNamespace::Value(call))
            }
            other => wrap_coerce(IrExpression::RuntimeCall(RuntimeNamespace::Value(other))),
        },
        IrExpression::Binary { op, left, right } => match op {
            IrBinOp::Add => value_binary(ValueCall::Add { left, right }),
            IrBinOp::Sub => value_number(ValueCall::Sub { left, right }),
            IrBinOp::Mul => value_number(ValueCall::Mul { left, right }),
            IrBinOp::Div => value_number(ValueCall::Div { left, right }),
            IrBinOp::Mod => value_number(ValueCall::Mod { left, right }),
            IrBinOp::Equal => value_bool(ValueCall::Equal { left, right }),
            IrBinOp::StrictEqual => value_bool(ValueCall::StrictEqual { left, right }),
            IrBinOp::NotEqual => value_bool(ValueCall::NotEqual { left, right }),
            IrBinOp::StrictNotEqual => value_bool(ValueCall::StrictNotEqual { left, right }),
            IrBinOp::LessThan => value_bool(ValueCall::LessThan { left, right }),
            IrBinOp::LessThanOrEqual => value_bool(ValueCall::LessThanOrEqual { left, right }),
            IrBinOp::GreaterThan => value_bool(ValueCall::GreaterThan { left, right }),
            IrBinOp::GreaterThanOrEqual => {
                value_bool(ValueCall::GreaterThanOrEqual { left, right })
            }
            _ => wrap_coerce(IrExpression::Binary { op, left, right }),
        },
        other => wrap_coerce(other),
    }
}

fn value_number(call: ValueCall) -> IrExpression {
    wrap_coerce(value_binary(call))
}

fn value_bool(call: ValueCall) -> IrExpression {
    wrap_coerce(value_binary(call))
}

fn wrap_coerce(expr: IrExpression) -> IrExpression {
    IrExpression::RuntimeCall(RuntimeNamespace::Value(ValueCall::Coerce {
        expr: Box::new(expr),
    }))
}

pub(crate) fn value_binary(call: ValueCall) -> IrExpression {
    IrExpression::RuntimeCall(RuntimeNamespace::Value(call))
}
