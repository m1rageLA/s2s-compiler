use std::path::Path;
use swc_common::{SourceMap, comments::SingleThreadedComments, sync::Lrc};
use swc_ecma_ast::Module;
use swc_ecma_parser::Parser;

mod lexer;

fn parse() -> Module {
    let comments: SingleThreadedComments = SingleThreadedComments::default();

    // SourceMap manages source files and resolves byte positions to source locations
    // It can inform us about exact position of Error, element, code etc.
    // In the next generation we can add linter using this SourceMap, because we will be able to locate exact 'heavy' function or part of code
    // OR we will be able to show users what is not suppoertd for now
    let source_map: Lrc<SourceMap> = Default::default();
    let program = source_map.load_file(Path::new("./ts.ts")).unwrap();

    // Lexer is a just list of tokens (parts of code like 'function', '(', ')', '{'...})
    let lexer = lexer::lexer(program.as_ref(), &comments);
    let mut parser = Parser::new_from(lexer);

    let _module = parser.parse_module().expect("failed to parser module");

    _module
}