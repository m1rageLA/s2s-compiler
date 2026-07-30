// Position of the code in the source file, like "loc" in estree spec.
// Each node will enhance with this Position struct, so we will be able to locate exact position of the node in the source file
// MVP for now:
// const x = 10;

mod declarations;
mod identifiers;    
mod expressions;

pub struct Position {
    line: usize,
    column: usize,
}

// struct Program { 
//     statements: Vec<Statement>,
//     position: Position,
// }

// enum Statement {
//     ExpressionStmt,
//     Directive
// }

// struct ExpressionStmt {
//     expression: Expression,
//     position: Position,
// }

// struct Directive {
//     expression: Expression,
//     directive: String,
//     position: Position,
// }

// struct Expression {}

// struct Indentifier {
//     name: String,
//     position: Position,
// }

// struct Literal {
//     value: String,
//     position: Position,
// }
