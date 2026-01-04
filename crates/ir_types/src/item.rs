use serde::{Deserialize, Serialize};

use crate::{IrExpression, IrStmt, IrType};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IrModule {
    pub items: Vec<IrItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IrItem {
    Variable(IrVariable),
    Function(IrFunction),
    Expression(IrExpression),
    Block(Vec<IrStmt>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IrVariable {
    pub name: String,
    pub mutable: bool,
    pub ty: IrType,
    pub value: Option<IrExpression>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IrFunction {
    pub name: String,
    pub params: Vec<IrParam>,
    pub ret: IrType,
    pub body: Vec<IrStmt>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IrParam {
    pub name: String,
    pub ty: IrType,
}
