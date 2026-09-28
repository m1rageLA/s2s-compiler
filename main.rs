use std::println;

use compiler::normalize_to_es5::normalize_to_es5;

use compiler::compile_and_execute::compile_and_execute;
fn main() {
    let source = r#"
const a = [1, 2];
const b = [3, 4];
const result = [...a, ...b];
    "#;

    let norm = normalize_to_es5(source);
    println!("\n\n{}\n\n", norm);

    let code = compile_and_execute(source);

    println!("{code}");
}
