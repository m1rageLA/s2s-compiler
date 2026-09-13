use crate::transform::module;

mod transform;

pub fn codegen(ast: swc_ecma_ast::Module) -> String {
    module::emit(ast).to_string()
}
