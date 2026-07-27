use std::{path::Path, println};
use swc_common::GLOBALS;
use swc_common::{Mark, SourceMap, comments::SingleThreadedComments, sync::Lrc};
use swc_ecma_ast::{EsVersion, Module, Pass, Program};
use swc_ecma_parser::{self, Parser};
use swc_ecma_preset_env::transform_from_es_version;
use swc_ecma_transforms_base::assumptions::Assumptions;

mod lexer;

fn parse() -> () {
    let comments: SingleThreadedComments = SingleThreadedComments::default();

    // SourceMap manages source files and resolves byte positions to source locations
    // It can inform us about exact position of Error, element, code etc.
    // In the next generation we can add linter using this SourceMap, because we will be able to locate exact 'heavy' function or part of code
    // OR we will be able to show users what is not suppoertd for now
    let source_map: Lrc<SourceMap> = Default::default();
    let program = source_map.load_file(Path::new("./ts.js")).unwrap();

    // Lexer is a just list of tokens (parts of code like 'function', '(', ')', '{'...})
    let lexer = lexer::lexer(program.as_ref(), &comments);
    let mut parser = Parser::new_from(lexer);
    let module = parser.parse_module().unwrap(); // TODO: handle error

    let mut program: Program = Program::Module(module);

    GLOBALS.set(&Default::default(), || {
        let unresolved_mark = Mark::new();
        let mut pass = transform_from_es_version(
            unresolved_mark,
            None::<SingleThreadedComments>,
            EsVersion::Es3,
            Assumptions::default(),
            false,
        );
        pass.process(&mut program);
        let module = match program {
            Program::Module(module) => module,
            Program::Script(_) => unreachable!(),
        };

        let js = swc_ecma_codegen::to_code(&module);
        println!("Normalized module: {}", js);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        swc_common::GLOBALS.set(&Default::default(), || {
            parse();
        });
    }
}
