#[derive(Debug, Clone, PartialEq)]
pub struct IrModule {
    pub items: Vec<IrItem>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IrItem {
    Variable(IrVariable),
    Function(IrFunction),
    Expression(IrExpression),
}

#[derive(Debug, Clone, PartialEq)]
pub struct IrVariable {
    pub name: String,
    pub ty: IrType,
    pub value: Option<IrExpression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IrFunction {
    pub name: String,
    pub params: Vec<IrParam>,
    pub ret: IrType,
    pub body: Vec<IrStmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IrStmt {
    Leteral(IrVariable),
    Expression(IrExpression),
    Return(Option<IrExpression>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct IrParam {
    pub name: String,
    pub ty: IrType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrType {
    Int,
    Str,
    Bool,
    Unit,
    Any,
}

#[derive(Debug, Clone, PartialEq)]
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
}

#[derive(Debug, Clone, PartialEq)]
pub enum IrLiteral {
    Int(i32),
    Str(String),
    Bool(bool),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    EqEq,       // ==
    NotEq,      // !=
    EqEqEq,     // ===
    NotEqEq,    // !==
    Lt,         // <
    LtEq,       // <=
    Gt,         // >
    GtEq,       // >=
    LShift,     // <<
    RShift,     // >>
    ZeroFillRShift, // >>>
    Add,        // +
    Sub,        // -
    Mul,        // *
    Div,        // /
    Mod,        // %
    BitOr,      // |
    BitXor,     // ^
    BitAnd,     // &
    LogicalOr,  // ||
    LogicalAnd, // &&
    In,         // in
    InstanceOf, // instanceof
    Exp,        // ** (возведение в степень)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrBinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Exp,

    Equal,               // ==
    StrictEqual,         // ===
    NotEqual,            // !=
    StrictNotEqual,      // !==

    LessThan,            // <
    LessThanOrEqual,     // <=
    GreaterThan,         // >
    GreaterThanOrEqual,  // >=

    LeftShift,           // <<
    RightShift,          // >>
    UnsignedRightShift,  // >>>

    BitwiseOr,           // |
    BitwiseXor,          // ^
    BitwiseAnd,          // &

    LogicalOr,           // ||
    LogicalAnd,          // &&

    In,                  // in
    InstanceOf,          // instanceof

    Unsupported,
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    F64,
    Bool,
    String,
    Unit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ident(pub String);
