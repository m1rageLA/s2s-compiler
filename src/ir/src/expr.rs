use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IrExpression {
    Identifier(String),
    Literal(IrLiteral),
    Binary {
        op: IrBinOp,
        left: Box<IrExpression>,
        right: Box<IrExpression>,
    },
    Call {
        callee: Box<IrExpression>,
        args: Vec<IrExpression>,
    },
    Array(Vec<IrExpression>),
    RuntimeCall(IrRuntimeCall),
    Member {
        object: Box<IrExpression>,
        property: String,
    },
    SuperCall {
        args: Vec<IrExpression>,
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IrLiteral {
    Int(i32),
    Str(String),
    Bool(bool),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IrRuntimeCall {
    pub kind: RuntimeNamespace,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RuntimeNamespace {
    Console(ConsoleCall),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConsoleCall {
    Log(Vec<IrExpression>),
    Error(Vec<IrExpression>),
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IrBinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Exp,

    Equal,
    StrictEqual,
    NotEqual,
    StrictNotEqual,

    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,

    LeftShift,
    RightShift,
    UnsignedRightShift,

    BitwiseOr,
    BitwiseXor,
    BitwiseAnd,

    LogicalOr,
    LogicalAnd,

    In,
    InstanceOf,

    Unsupported,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinaryOp {
    EqEq,
    NotEq,
    EqEqEq,
    NotEqEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    LShift,
    RShift,
    ZeroFillRShift,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    BitOr,
    BitXor,
    BitAnd,
    LogicalOr,
    LogicalAnd,
    In,
    InstanceOf,
    Exp,
}
