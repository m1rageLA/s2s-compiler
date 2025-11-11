pub mod parser;
pub mod normalizer;
use swc_ecma_ast::Module;
use crate::normalizer::ast_normalize;
use crate::parser::ts_to_ast;

pub fn ast(source: &str) -> Module {
    let ast = ts_to_ast(source);
    let ast_norm = ast_normalize(ast);
    ast_norm
}
