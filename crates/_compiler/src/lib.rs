use logger::Logger;
use lowering::lowering;
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
    Logger::success("finished parsing", "_compiler");

    // let ir = lower(normalized_ast);
    // Logger::success("ast-module to ir", "compiler");

    // let hir = hir::lower(normalized_ast);
    // Logger::success("ast-module to hir", "compiler");

    let ir = lowering(normalized_ast);
    Logger::success("finished lowering", "_compiler");
}
