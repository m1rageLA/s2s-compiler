pub mod parser;
pub mod normalizer;
mod validator;

use swc_ecma_ast::Module;

use crate::normalizer::ast_normalize;
use crate::parser::ts_to_ast;
use crate::validator::assert_es5_strict;

pub fn ast(source: &str) -> Module {
    let ast = ts_to_ast(source);
    let ast_norm = ast_normalize(ast);
    assert_es5_strict(&ast_norm);
    ast_norm
}
