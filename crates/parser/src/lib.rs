use swc_common::errors::EmitterWriter;
use swc_common::errors::Handler;
use swc_common::sync::Lrc;
use swc_common::SourceMap;
use swc_ecma_ast::Module;
use swc_ecma_parser::TsSyntax;
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};

pub fn ast(sourcecode: &str) -> Module {
    let cm: Lrc<SourceMap> = Default::default();
    let emitter = EmitterWriter::new(Box::new(std::io::stderr()), Some(cm.clone()), true, true);
    let handler = Handler::with_emitter(true, false, Box::new(emitter));

    let fm = cm.new_source_file(
        Lrc::new(swc_common::FileName::Custom("test.ts".into())),
        sourcecode.to_string(),
    );

    let lexer = Lexer::new(
        Syntax::Typescript(TsSyntax::default()),
        Default::default(),
        StringInput::from(&*fm),
        None,
    );

    let mut parser = Parser::new_from(lexer);

    let _module: Module = parser
        .parse_module()
        .map_err(|e| e.into_diagnostic(&handler).emit())
        .expect("failed to parser module");
    _module
}
