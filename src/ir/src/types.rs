#[derive(Debug, Clone)]
pub enum Type {
    F64,
    Bool,
    String,
    Unit,
}

#[derive(Debug, Clone)]
pub struct Ident(pub String);

#[derive(Debug, Clone)]
pub enum BinOp {
    Add,
    Multiply,
    Divide,
    Subtract,
    Equals,
    NotEquals,
    LessThan,
    LessThanEquals,
    GreaterThan,
    GreaterThanEquals,
    And,
    Or,
    Xor,
    Not,
}

