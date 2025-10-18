use serde::{Deserialize, Serialize};

use crate::{IrParam, IrStmt, IrType};

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
    Arrow {
        params: Vec<IrParam>,
        body: IrArrowBody,
    },
    RuntimeCall(RuntimeNamespace),
    Member {
        object: Box<IrExpression>,
        property: String,
    },
    Template(Vec<IrTemplatePart>),
    SuperCall {
        args: Vec<IrExpression>,
    },
    Conditional {
        test: Box<IrExpression>,
        consequent: Box<IrExpression>,
        alternate: Box<IrExpression>,
    },
    ArrayExpr(Vec<IrExpression>),
    Function(Box<IrFunctionExpr>),
    PostfixUnary {
        left: Box<IrExpression>,
        op: IrPostfixOp,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IrArrowBody {
    Expr(Box<IrExpression>),
    Block(Vec<IrStmt>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IrPostfixOp {
    Increment,
    Decrement,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IrFunctionExpr {
    pub name: Option<String>,
    pub params: Vec<IrParam>,
    pub ret: IrType,
    pub body: Vec<IrStmt>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IrLiteral {
    Number(f64),
    Str(String),
    Bool(bool),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RuntimeNamespace {
    Console(ConsoleCall),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConsoleCall {
    Log(Vec<IrExpression>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IrTemplatePart {
    String(String),
    Expr(Box<IrExpression>),
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
