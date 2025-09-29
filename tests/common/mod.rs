// Общие хелперы для всех тестов
use swc_common::{FileName, SourceMap};
use swc_ecma_ast as ast;
use swc_ecma_parser::{Parser, StringInput, Syntax, TsSyntax, lexer::Lexer};

pub fn parse_ts_module(src: &str) -> ast::Module {
    let cm = SourceMap::default();
    let fm = cm.new_source_file(FileName::Custom("test.ts".into()).into(), src.to_string());

    let lexer = Lexer::new(
        Syntax::Typescript(TsSyntax {
            tsx: false,
            dts: false,
            ..Default::default()
        }),
        Default::default(),
        StringInput::from(&*fm),
        None,
    );

    let mut parser = Parser::new_from(lexer);
    parser.parse_module().expect("failed to parse module")
}
