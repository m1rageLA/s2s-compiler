use std::env;
use std::process::Command;

mod input;
mod pipeline;

pub(crate) const DEFAULT_TS_PATH: &str = "fixtures/ts/program.ts";
pub(crate) const GENERATED_CARGO_MANIFEST: &str = r#"[package]
name = "generated"
version = "0.1.0"
edition = "2021"

[dependencies]
runtime = { path = "../core/crates/runtime" }

[workspace]
"#;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    //entry point of compiler
    if args.len() > 1 {
        match args[1].as_str() {
            "ast" => {
                pipeline::run_ast();
                return;
            }
            "ir" => {
                pipeline::run_ir();
                return;
            }
            _ => {
                pipeline::run_pipeline();
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

    pipeline::run_pipeline();
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
