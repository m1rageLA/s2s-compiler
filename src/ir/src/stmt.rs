use serde::{Deserialize, Serialize};

use crate::{IrExpression, IrVariable};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IrStmt {
    Leteral(IrVariable),
    Expression(IrExpression),
    Return(Option<IrExpression>),
    Block(Vec<IrStmt>),
    While(IrExpression, Vec<IrStmt>),
    VarDecl(Vec<IrVariable>),
    Unsupported(String),
}
