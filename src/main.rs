use codegen::{Codegen, ModuleGenerator};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::Instant;

const DEFAULT_TS_PATH: &str = "ts/program.ts";

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "ast" => {
                run_ast();
                return;
            }
            _ => 
            { 
                run_pipeline();
                return;
            }
        }
    }

    if cfg!(debug_assertions) {
        println!("[debug] Detected debug build; launching release run for final output...");
        match launch_release_run() {
            Ok(status) if status.success() => return,
            Ok(status) => {
                if let Some(code) = status.code() {
                    std::process::exit(code);
                } else {
                    std::process::exit(1);
                }
            }
            Err(err) => {
                eprintln!("[error] Failed to start release run: {err}");
                std::process::exit(1);
            }
        }
    }

    run_pipeline();
}

fn run_ast() {
    let (ts_path, ts_source) = match load_typescript_source() {
        Ok(result) => result,
        Err(err) => {
            eprintln!("    [error] {err}");
            std::process::exit(1);
        }
    };
    let ast = parser::ast(&ts_source);
    println!("{:#?}", ast);
}
fn run_pipeline() {
    let total_stages = 5;
    let pipeline_start = Instant::now();

    // println!("=== TypeScript → Rust Compilation Pipeline ===");

    // log_stage(1, total_stages, "Loading TypeScript source file");
    let load_timer = Instant::now();
    let (ts_path, ts_source) = match load_typescript_source() {
        Ok(result) => result,
        Err(err) => {
            eprintln!("    [error] {err}");
            std::process::exit(1);
        }
    };
    // println!(
    //     "    -> Loaded {} bytes from {}",
    //     ts_source.len(),
    //     ts_path.display()
    // );
    // println!(
    //     "    [ok] Source file loaded in {:.2?}",
    //     load_timer.elapsed()
    // );

    // log_stage(2, total_stages, "Parsing source into AST");
    let ast_timer = Instant::now();
    let ast = parser::ast(&ts_source);

    // log_stage(3, total_stages, "Lowering AST to intermediate representation");
    let lowering_timer = Instant::now();
    let ir_module = lowering::ast_to_ir(&ast);
    // println!(
    //     "    [ok] Lowering produced {} module item(s) in {:.2?}",
    //     ir_module.items.len(),
    //     lowering_timer.elapsed()
    // );

    // log_stage(4, total_stages, "Generating Rust code from IR");
    let generation_timer = Instant::now();
    let mut generator = ModuleGenerator::new();
    let mut printed_header = false;
    for item in &ir_module.items {
        let element = item.codegen();
        let element_text = element.to_string();
        if !element_text.trim().is_empty() {
            if !printed_header {
                // println!("    -> Generated Rust module:");
                printed_header = true;
            }
            for line in element_text.lines() {
                println!("       {}", line);
            }
        }
        generator.add_element(element);
    }
    let rust_code = generator.finish().to_string();
    // println!(
    //     "    [ok] Rust generation completed in {:.2?}",
    //     generation_timer.elapsed()
    // );

    // log_stage(5, total_stages, "Compiling and running generated program (--release)");
    let execution_timer = Instant::now();
    match run_generated(&rust_code) {
        Ok(stdout) => {
            let trimmed = stdout.trim();
            if trimmed.is_empty() {
                // println!("    -> Generated program produced no stdout");
            } else {
                // println!("    -> Generated program output:");
                for line in trimmed.lines() {
                    println!("       {}", line);
                }
            }
            // println!(
            //     "    [ok] Execution finished in {:.2?}",
            //     execution_timer.elapsed()
            // );
        }
        Err(err) => {
            eprintln!(
                "    [error] Generated program failed:\n{}",
                indent_multiline(&err)
            );
            std::process::exit(1);
        }
    }

    // println!(
    //     "=== Pipeline completed successfully in {:.2?} ===",
    //     pipeline_start.elapsed()
    // );
}

fn load_typescript_source() -> Result<(PathBuf, String), String> {
    let path_arg = env::args().nth(2);
    let path = path_arg
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(DEFAULT_TS_PATH));
    let contents =
        fs::read_to_string(&path).map_err(|e| format!("Failed to read {}: {e}", path.display()))?;
    Ok((path, contents))
}

fn run_generated(rust: &str) -> Result<String, String> {
    // println!("    -> Writing generated Rust to out/src/main.rs");
    fs::create_dir_all("out/src").map_err(|e| format!("create out dir failed: {e}"))?;
    fs::write("out/src/main.rs", rust).map_err(|e| format!("write out/src/main.rs failed: {e}"))?;

    // println!("    -> Running cargo run --release in ./out");
    let run = Command::new("cargo")
        .args(["run", "--quiet", "--release"])
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

fn launch_release_run() -> std::io::Result<std::process::ExitStatus> {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut command = Command::new("cargo");
    command.arg("run").arg("--release");
    if !args.is_empty() {
        command.arg("--");
        command.args(&args);
    }
    command.status()
}

fn log_stage(step: usize, total: usize, message: &str) {
    // println!("[{step}/{total}] {message}");
}

fn indent_multiline(message: &str) -> String {
    message
        .lines()
        .map(|line| format!("       {line}"))
        .collect::<Vec<_>>()
        .join("\n")
}
