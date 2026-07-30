use crate::Position;

pub enum Expression {
    AssignmentExpression,
    BinaryExpression,
   Literal,
}

enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    EqEq,
    NotEq,
    Gt,
    GtEq,
    Lt,
    LtEq,
    // TODO: implement in the next iteration
}

enum AssignmentOperator {
    Eq,
    PlusEq,
    MinusEq,
    MulEq,
    DevEq,
    ModEq,
    // TODO: implement in the next iteration
}

struct AssignmentExpression {
    operator: AssignmentOperator,
    left: Expression,
    right: Expression,
    position: Position,
}

enum ValueTypes {
    String, 
    Boolean
}

struct Literal {
    value: ValueTypes,   
    position: Position,
}