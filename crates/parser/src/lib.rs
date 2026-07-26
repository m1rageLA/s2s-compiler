
use std::path::Path;
use std::println;
use swc_common::comments::SingleThreadedComments;
use swc_common::sync::Lrc;
use swc_common::{
    FileName, SourceMap,
};
use swc_ecma_codegen;
use swc_ecma_parser::{Lexer, Parser, StringInput, Syntax, TsSyntax};

fn parse() -> () {
    let comments = SingleThreadedComments::default();
    // SourceMap manages source files and resolves byte positions to source locations
    // It can inform us about exact position of Error, element, code etc.
    // In the next generation we can add linter using this SourceMap, because we will be able to locate exact 'heavy' function or part of code
    // OR we will be able to show users what is not suppoertd for now
    let source_map: Lrc<SourceMap> = Default::default();
    let program = source_map.load_file(Path::new("./ts.ts")).unwrap();
    // Lexer is a just list of tokens (parts of code like 'function', '(', ')', '{'...}) 
    let lexer = Lexer::new(
        Syntax::Typescript(TsSyntax::default()),
        swc_ecma_ast::EsVersion::Es5,
        StringInput::from(program.as_ref()),
        Some(&comments),
    );

    let mut parser = Parser::new_from(lexer);
    
        let _module = parser
        .parse_module()
        .expect("failed to parser module");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        parse();
    }
}
