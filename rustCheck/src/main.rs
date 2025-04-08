use std::fs;
use ra_ap_syntax::{AstNode, SourceFile, SyntaxNode};

// 1. Import Serde traits for JSON serialization
use serde::Serialize;

#[derive(Debug, Serialize)]
struct SimpleAST {
    kind: String,
    text: String,
    children: Vec<SimpleAST>,
}

fn parse_code_to_ast(file_name: &str) -> SimpleAST {
    let source_code = fs::read_to_string(file_name)
        .expect("Failed to read the specified file.");

    let parsed_file = SourceFile::parse(&source_code);
    let syntax_node = parsed_file.tree().syntax().clone();

    fn to_simple_ast(node: SyntaxNode, full_text: &str) -> SimpleAST {
        let kind = format!("{:?}", node.kind());
        let range = node.text_range();
        let text = &full_text[range.start().into()..range.end().into()];

        let children = node
            .children()
            .map(|child| to_simple_ast(child, full_text))
            .collect();

        SimpleAST {
            kind,
            text: text.to_string(),
            children,
        }
    }

    to_simple_ast(syntax_node, &source_code)
}

fn main() {
    // Adjust the path to wherever your example Rust file is located:
    let ast = parse_code_to_ast("example.rs");

    // 2. Serialize to JSON (pretty-printed) and print
    let json_output = serde_json::to_string_pretty(&ast)
        .expect("Failed to serialize AST to JSON");

    println!("{}", json_output);
}
