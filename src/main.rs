use codegen::{Codegen, ModuleGenerator};
use std::fs;
use std::process::{Command, Stdio};

fn main() {
    let code = "
    
    let x: number = 10
    console.log(`${x}`);
";

    // 2) фронтенд твоего компилятора
    //    (парсим один раз и по цепочке)
    let ast = parser::ast(code);
    let ir_module = lowering::ast_to_ir(&ast);
    // println!("ir_module: {:#?}", ir_module);
    let mut generator = ModuleGenerator::new();
    for item in &ir_module.items {
        let element = item.codegen();
        if !element.is_empty() {
            println!("{}", element);
        }
        generator.add_element(element);
    }

    let ts = generator.finish();

    // 3) код как строка (то, что будем компилировать rustc)
    let rust_code = ts.to_string();
    // println!("======1111===={}", rust_code);

    // 4) компилируем и запускаем
    match run_generated(&rust_code) {
        Ok(stdout) => {
            println!("Program stdout:\n{}", stdout);
        }
        Err(err) => {
            eprintln!("Failed to compile/run generated code:\n{}", err);
        }
    }
}

fn run_generated(rust: &str) -> Result<String, String> {
    fs::create_dir_all("out/src").map_err(|e| format!("create out dir failed: {e}"))?;
    // сохранить файл
    fs::write("out/src/main.rs", rust)
        .map_err(|e| format!("write out/src/main.rs failed: {e}"))?;

    // собрать и запустить сгенерированный проект в `out`
    let run = Command::new("cargo")
        .args(["run", "--quiet"])
        .current_dir("out")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| format!("failed to run cargo for generated code: {e}"))?;

    if !run.status.success() {
        let err = String::from_utf8_lossy(&run.stderr);
        return Err(format!("generated program failed:\n{}", err));
    }

    Ok(String::from_utf8_lossy(&run.stdout).into_owned())
}
