use swc_common::{SourceFile, comments::SingleThreadedComments};
use swc_ecma_parser::{Lexer, StringInput, Syntax, TsSyntax};

pub fn lexer<'a>(source: &'a SourceFile, comments: &'a SingleThreadedComments) -> Lexer<'a> {
    let lexer: Lexer<'_> = Lexer::new(
        Syntax::Es(Default::default()),
        swc_ecma_ast::EsVersion::Es3,
        StringInput::from(source),
        Some(comments),
    );

    lexer
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;
    use swc_common::FileName;
    use swc_common::SourceMap;
    use swc_common::sync::Lrc;

    #[test]
    fn it_converts_sourcefile_to_tokens() {
        let comments: SingleThreadedComments = SingleThreadedComments::default();
        let source_map: Lrc<SourceMap> = Default::default();
        let program = source_map.new_source_file(
            FileName::Custom("test.js".into()).into(),
            "function foo() {return 5}",
        );
        // Lexer is a just list of tokens (parts of code like 'function', '(', ')', '{'...})
        let lexer = lexer(program.as_ref(), &comments);
        let count = lexer.into_iter().count();
        assert!(count > 0, "Expected more than 0 tokens, but got {}", count);
        assert_eq!(count, 8, "Expected 8 tokens, but got {}", count);
    }
}
