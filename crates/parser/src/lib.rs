pub mod parser;
pub mod normalizer;
mod validator;

use swc_ecma_ast::Module;

use crate::parser::ts_to_ast;

pub fn ast(source: &str) -> Module {
    ts_to_ast(source)
}
