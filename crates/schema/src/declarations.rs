use crate::{Position, expressions::Expression, identifiers::Identifier};

pub struct VariableDeclaration {
    identifier: Identifier,
    initializer: Option<Expression>,
    position: Position,
}