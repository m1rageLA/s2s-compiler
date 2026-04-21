use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IrType {
    Number,
    UInt,
    Str,
    Bool,
    Unit,
    Any,
    Value,
    Array(IrArrayKind),
    Object(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IrArrayKind {
    Unknown,
    Any,
    Value,
    Number,
    Str,
    Bool,
    Object(u32),
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

impl IrType {
    pub fn is_numeric(self) -> bool {
        matches!(self, IrType::Number | IrType::UInt)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IrObjectField {
    pub name: String,
    pub ty: IrType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IrTypeAliasDef {
    Object(Vec<IrObjectField>),
    Alias(IrType),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IrTypeAlias {
    pub id: u32,
    pub name: String,
    pub def: IrTypeAliasDef,
}
