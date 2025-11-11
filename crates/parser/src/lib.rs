use swc_common::sync::Lrc;
use swc_common::{FileName, SourceMap};
use swc_ecma_ast::Module;
use swc_ecma_parser::{Lexer, Parser, StringInput, Syntax, TsSyntax};

pub fn ast(source: &str) -> Module {
    let cm = SourceMap::default();
    let fm = cm.new_source_file(
        Lrc::new(FileName::Custom("input.ts".into())),
        source.to_owned(),
    );
    let syntax = Syntax::Typescript(TsSyntax {
        tsx: false,
        decorators: true,
        ..Default::default()
    });
    let lexer = Lexer::new(syntax, Default::default(), StringInput::from(&*fm), None);
    let mut parser = Parser::new_from(lexer);
    let module = parser.parse_module().expect("failed to parse module");

    module
}
