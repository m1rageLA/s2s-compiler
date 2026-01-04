use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IrType {
    Number,
    Str,
    Bool,
    Unit,
    Any,
    Value,
    Array(IrArrayKind),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IrArrayKind {
    Unknown,
    Any,
    Value,
    Number,
    Str,
    Bool,
}

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
