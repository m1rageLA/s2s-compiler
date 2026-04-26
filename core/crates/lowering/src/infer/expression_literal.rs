use ir::{IrLiteral, IrType};

pub(crate) fn infer_literal(literal: &IrLiteral) -> Option<IrType> {
    match literal {
        IrLiteral::Number(value) => Some(literal_number_type(*value)),
        IrLiteral::Str(_) => Some(IrType::Str),
        IrLiteral::Bool(_) => Some(IrType::Bool),
        IrLiteral::Null => Some(IrType::Value),
    }
}

fn literal_number_type(value: f64) -> IrType {
    if value.is_finite() && value >= 0.0 && value.fract() == 0.0 {
        IrType::UInt
    } else {
        IrType::Number
    }
}
