use std::sync::Arc;
use swc_common::errors::EmitterWriter;
use swc_common::errors::Handler;
use swc_common::sync::Lrc;
use swc_common::{errors::ColorConfig, FilePathMapping, SourceMap, Span};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};

fn main() {
    let cm: Lrc<SourceMap> = Default::default();
    let emitter = EmitterWriter::new(Box::new(std::io::stderr()), Some(cm.clone()), true, true);
    let handler = Handler::with_emitter(true, false, Box::new(emitter));

    let fm = cm.new_source_file(
        Lrc::new(swc_common::FileName::Custom("test.ts".into())),
        "let x = 1",
    );

    let span = Span::new(fm.start_pos, swc_common::BytePos(fm.start_pos.0 + 3));
    let pos = cm.lookup_char_pos(span.lo());
    println!(
        "File {}, Line {}, Column {}",
        pos.file.name, pos.line, pos.col_display
    );

    handler.struct_span_err(span, "test").emit();
}
