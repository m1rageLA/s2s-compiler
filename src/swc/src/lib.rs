use std::sync::Arc;
use swc_common::errors::EmitterWriter;
use swc_common::errors::Handler;
use swc_common::sync::Lrc;
use swc_common::{errors::ColorConfig, FilePathMapping, SourceMap, Span};
use swc_ecma_parser::EsSyntax;
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};

pub fn ast() {
    let cm: Lrc<SourceMap> = Default::default();
    let emitter = EmitterWriter::new(Box::new(std::io::stderr()), Some(cm.clone()), true, true);
    let handler = Handler::with_emitter(true, false, Box::new(emitter));

    let fm = cm.new_source_file(
        Lrc::new(swc_common::FileName::Custom("test.ts".into())),
        "let x = 1",
    );

    let lexer = Lexer::new(
        Syntax::Es(Default::default()),
        Default::default(),
        StringInput::from(&*fm),
        None,
    );

    let mut parser = Parser::new_from(lexer);

    let _module = parser
        .parse_module()
        .map_err(|e| e.into_diagnostic(&handler).emit())
        .expect("failed to parser module");

    println!("{:#?}", _module);
}
