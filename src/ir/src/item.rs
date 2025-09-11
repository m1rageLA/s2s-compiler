#[derive(Debug)]
pub struct IrModule {
    pub items: Vec<IrItem>,
}

#[derive(Debug)]
pub enum IrItem {
    Variable(IrVariable),
    Function(IrFunction),
}

#[derive(Debug)]
pub struct IrVariable {
    pub name: String,
    pub ty: IrType,
    pub value: Option<IrExpression>,
}

#[derive(Debug)]
pub struct IrFunction {
    pub name: String,
    pub params: Vec<IrParam>,
    pub body: Vec<IrStmt>,
}

#[derive(Debug)]
pub enum IrStmt {
    Leteral(IrVariable),
    Expression(IrExpression),
    Return(Option<IrExpression>),
}

#[derive(Debug)]
pub struct IrParam {
    pub name: String,
    pub ty: IrType,
}

#[derive(Debug)]
pub enum IrType {
    Int,
    Str,
    Bool,
    Unit,
    Any,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum IrLiteral {
    Int(i32),
    Str(String),
    Bool(bool),
}

#[derive(Debug)]
pub enum IrBinOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Neq,
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

