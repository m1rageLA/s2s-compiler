use logger::Logger;
use swc_common::{FileName, SourceMap, comments::SingleThreadedComments, sync::Lrc};
use swc_ecma_ast::Module;
use swc_ecma_parser::{self, Parser};

mod lexer;
mod normalizer;

pub fn parse(source_code: &str) -> Module {
    let comments: SingleThreadedComments = SingleThreadedComments::default();
    // SourceMap manages source files and resolves byte positions to source locations
    // It can inform us about exact position of Error, element, code etc.
    // In the next generation we can add linter using this SourceMap, because we will be able to locate exact 'heavy' function or part of code
    // OR we will be able to show users what is not suppoertd for now
    let source_map: Lrc<SourceMap> = Default::default();
    let program = source_map.new_source_file(
        FileName::Custom("input.ts".into()).into(),
        source_code.to_owned(),
    );

    // Lexer is a just list of tokens (parts of code like 'function', '(', ')', '{'...})
    let lexer = lexer::lexer(program.as_ref(), &comments);
    let mut parser = Parser::new_from(lexer);
    let ast = parser.parse_module().unwrap(); // TODO: handle error

    let normalized_ast = normalizer::normalizer(ast);

    Logger::step("return normalized ast", "parser");
    normalized_ast
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        swc_common::GLOBALS.set(&Default::default(), || {
            // parse();
        });
    }
}
