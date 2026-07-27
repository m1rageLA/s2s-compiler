use logger::Logger;
use parser::parse;

pub fn compileToRust() {
    let source_code = r#"

        function add(a, b) {
            return a + b;
        }

    "#;

    // Parse the source code into an AST module
    // string -> Module
    let normalized_ast = parse(source_code);
    Logger::success("ast-module to rust-code", "compiler");

    // let ir = lower(normalized_ast);
    // Logger::success("ast-module to ir", "compiler");

    // let hir = hir::lower(normalized_ast);
    // Logger::success("ast-module to hir", "compiler");

    // let rust_code = codegen(normalized_ast);
    // Logger::success("ir to rust-code", "compiler");
}