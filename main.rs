use std::println;

use compiler::normalize_to_es5::normalize_to_es5;

use compiler::compile_and_execute::compile_and_execute;

fn main() {
    let source = r#"
        const x = [1, 2, 3];
    "#;

    let norm = normalize_to_es5(source);
    println!("\n\n{}\n\n", norm);

    let code = compile_and_execute(source);

    println!("{code}");
}