use codegen::codegen;
use logger::Logger;
use parser::parse;

pub fn compile_and_execute(source: &str) -> String {
    // Parse the source code into an AST module
    // string -> Module
    let normalized_ast = parse(source);

    // let ir = lower(normalized_ast);
    // Logger::success("ast-module to ir", "compiler");

    // let hir = hir::lower(normalized_ast);
    // Logger::success("ast-module to hir", "compiler");

    // let ir = lowering(normalized_ast).unwrap();

    let rust = codegen(normalized_ast);
    Logger::success("finished compiling", "_compiler");

    rust
}

#[cfg(test)]
mod tests {
    use super::compile_and_execute;

    #[test]
    fn codegen_handles_es5_normalized_do_while() {
        let output = compile_and_execute("do { [1]; } while (false);");

        assert!(output.contains("loop"), "unexpected output: {output}");
        assert!(
            output.contains("if ! (false)"),
            "unexpected output: {output}"
        );
    }

    #[test]
    fn codegen_handles_es5_normalized_for_in() {
        let output = compile_and_execute("for (var key in [10, 20]) { [key]; }");

        assert!(
            output.contains("for __compiler_for_in_index"),
            "unexpected output: {output}"
        );
        assert!(
            output.contains("let mut key"),
            "unexpected output: {output}"
        );
        assert!(output.contains("to_string"), "unexpected output: {output}");
    }
}
