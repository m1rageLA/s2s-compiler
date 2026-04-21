use serde::{Deserialize, Serialize};

use crate::{IrExpression, IrTypeAlias, IrVariable};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IrStmt {
    Leteral(IrVariable),
    Expression(IrExpression),
    Return(Option<IrExpression>),
    Block(Vec<IrStmt>),
    Empty,
    TypeAlias(IrTypeAlias),
    Labeled {
        label: String,
        body: Box<IrStmt>,
    },
    Break(Option<String>),
    Continue(Option<String>),
    If {
        condition: IrExpression,
        then_branch: Vec<IrStmt>,
        else_branch: Option<Vec<IrStmt>>,
    },
    While(IrExpression, Vec<IrStmt>),
    DoWhile(Vec<IrStmt>, IrExpression),
    For {
        init: Option<IrForInit>,
        condition: Option<IrExpression>,
        update: Option<IrExpression>,
        body: Vec<IrStmt>,
    },
    ForIn {
        left: IrForInLeft,
        right: IrExpression,
        body: Vec<IrStmt>,
    },
    Throw(IrExpression),
    VarDecl(Vec<IrVariable>),
    Switch {
        discriminant: IrExpression,
        cases: Vec<IrSwitchCase>,
    },
    Try {
        try_block: Vec<IrStmt>,
        catch: Option<IrCatchClause>,
        finally: Option<Vec<IrStmt>>,
    },
    Unsupported(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IrForInit {
    VarDecl(Vec<IrVariable>),
    Expr(IrExpression),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IrSwitchCase {
    pub test: Option<IrExpression>,
    pub consequent: Vec<IrStmt>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IrCatchClause {
    pub param: Option<String>,
    pub body: Vec<IrStmt>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IrForInLeft {
    Var(IrVariable),
    Identifier(String),
    Pattern(IrExpression),
}
