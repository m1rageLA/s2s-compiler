use ir::{IrLiteral, IrType};

pub(crate) fn infer_literal(literal: &IrLiteral) -> Option<IrType> {
    match literal {
        IrLiteral::Number(_) => Some(IrType::Number),
        IrLiteral::Str(_) => Some(IrType::Str),
        IrLiteral::Bool(_) => Some(IrType::Bool),
    }
}
