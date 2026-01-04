use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use ts2rust_core::lower_ast;
use ts2rust_core::prelude::Codegen;
use ts2rust_core::{compile_typescript, parse_typescript};

const DEFAULT_TS_PATH: &str = "ts/program.ts";
const GENERATED_CARGO_MANIFEST: &str = r#"[package]
name = "generated"
version = "0.1.0"
edition = "2021"

[dependencies]
runtime = { path = "../crates/runtime" }

[workspace]
"#;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "ast" => {
                run_ast();
                return;
            }
            "ir" => {
                run_ir();
                return;
            }
            // "js" => {
            //     run_js();
            //     return;
            // }
            _ => {
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
    let ts_source = match load_typescript_source() {
        Ok(result) => result,
        Err(err) => {
            eprintln!("    [error] {err}");
            std::process::exit(1);
        }
    };
    let ast = parse_typescript(&ts_source);
    println!("{:#?}", ast);
}

fn run_ir() {
    let ts_source = match load_typescript_source() {
        Ok(result) => result,
        Err(err) => {
            eprintln!("    [error] {err}");
            std::process::exit(1);
        }
    };
    let ir = lower_ast(&parse_typescript(&ts_source));
    println!("{:#?}", ir);
}

// fn run_js() {
//     let ts_source = match load_typescript_source() {
//         Ok(result) => result,
//         Err(err) => {
//             eprintln!("    [error] {err}");
//             std::process::exit(1);
//         }
//     };
//     let js = ts2rust_core::normalize_js(&ts_source);
//     println!("{js}");
// }
fn run_pipeline() {
    let ts_source = match load_typescript_source() {
        Ok(result) => result,
        Err(err) => {
            eprintln!("    [error] {err}");
            std::process::exit(1);
        }
    };
    let compilation = compile_typescript(&ts_source);
    let ir_module = &compilation.ir;
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
    }
    let rust_code = compilation.rust_string();
    match run_generated(&rust_code) {
        Ok(stdout) => {
            let trimmed = stdout.trim();
            if trimmed.is_empty() {
                // println!("    -> Generated program produced no stdout");
            } else {
                for line in trimmed.lines() {
                    println!("       {}", line);
                }
            }
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
    // );
}

fn load_typescript_source() -> Result<String, String> {
    let path_arg = env::args().nth(2);
    let path: PathBuf = path_arg
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(DEFAULT_TS_PATH));
    fs::read_to_string(&path).map_err(|e| format!("Failed to read {}: {e}", path.display()))
}

fn run_generated(rust: &str) -> Result<String, String> {
    // println!("    -> Writing generated Rust to out/src/main.rs");
    ensure_generated_manifest().map_err(|e| format!("failed to prepare out/Cargo.toml: {e}"))?;
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

fn ensure_generated_manifest() -> std::io::Result<()> {
    fs::create_dir_all("out")?;
    let manifest_path = Path::new("out/Cargo.toml");
    if manifest_path.exists() {
        let current = fs::read_to_string(manifest_path)?;
        if current == GENERATED_CARGO_MANIFEST {
            return Ok(());
        }
    }
    fs::write(manifest_path, GENERATED_CARGO_MANIFEST)
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

fn indent_multiline(message: &str) -> String {
    message
        .lines()
        .map(|line| format!("       {line}"))
        .collect::<Vec<_>>()
        .join("\n")
}
