use std::fs;
use std::process::{Command, Stdio};

fn main() {
    // 1) входные данные TS (пример)
    let code = "let x: number;";

    // 2) фронтенд твоего компилятора
    //    (парсим один раз и по цепочке)
    let ast = parser::ast(code);
    let ir = lowering::ast_to_ir(&ast);
    let ts = codegen::gen_module(&ir); // proc_macro2::TokenStream

    // 3) код как строка (то, что будем компилировать rustc)
    let rust_code = ts.to_string();

    // 4) компилируем и запускаем
    match run_generated(&rust_code) {
        Ok(stdout) => {
            println!("Program stdout:\n{}", stdout);
        }
        Err(err) => {
            eprintln!("Failed to compile/run generated code:\n{}", err);
        }
    }

    // 5) по желанию — вывести сам сгенерированный Rust
    println!("--- generated.rs ---\n{}", rust_code);
}

/// Компилирует `rust` с помощью `rustc` и запускает бинарь.
/// Возвращает stdout программы или подробную ошибку со stderr компилятора.
fn run_generated(rust: &str) -> Result<String, String> {
    // сохранить файл
    // fs::write("out.rs", rust).map_err(|e| format!("write out.rs failed: {e}"))?;

    // имя бинаря c учётом Windows
    let bin = if cfg!(windows) { "out_bin.exe" } else { "out_bin" };

    // скомпилировать
    let compile = Command::new("rustc")
        .arg("out.rs")
        .arg("-o")
        .arg(bin)
        // полезно указать явную редакцию, если внутри кода понадобиться:
        // .args(["--edition", "2021"])
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| format!("failed to spawn rustc: {e}"))?;

    if !compile.status.success() {
        let err = String::from_utf8_lossy(&compile.stderr);
        return Err(format!("rustc failed:\n{}", err));
    }

    // запустить бинарь
    let run = Command::new(format!("./{}", bin))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| format!("failed to run {}: {e}", bin))?;

    if !run.status.success() {
        let err = String::from_utf8_lossy(&run.stderr);
        return Err(format!("program failed:\n{}", err));
    }

    Ok(String::from_utf8_lossy(&run.stdout).into_owned())
}
