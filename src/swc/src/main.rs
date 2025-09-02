use std::sync::Arc;
use swc_common::errors::EmitterWriter;
use swc_common::errors::Handler;
use swc_common::sync::Lrc;
use swc_common::{errors::ColorConfig, FilePathMapping, SourceMap};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};

fn main() {
    let cm: Lrc<SourceMap> = Default::default();
}
