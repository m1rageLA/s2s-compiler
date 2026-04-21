#![deny(clippy::all)]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use napi::bindgen_prelude::*;
use napi_derive::napi;
use ts2rust_core::{compile_typescript, Compilation};
use uuid::Uuid;

const GENERATED_PACKAGE_NAME: &str = "generated";
const GENERATED_EDITION: &str = "2021";

#[napi(object)]
pub struct HeavyResult {
    pub stdout: String,
    pub rust: String,
}

/// Compile and execute a TypeScript snippet via the Rust toolchain.
#[napi]
pub fn heavy(source: String) -> AsyncTask<HeavyTask> {
    AsyncTask::new(HeavyTask { source })
}

pub struct HeavyTask {
    source: String,
}

impl Task for HeavyTask {
    type Output = HeavyResult;
    type JsValue = HeavyResult;

    /// Compile the given TypeScript source code into a Rust module, and then run the resulting
    /// program via the Rust toolchain.
    ///
    /// Returns a `HeavyResult` containing the stdout of the generated program and the generated
    /// Rust code.
    ///
    /// # Errors
    ///
    /// This function will return an error if the compilation of the TypeScript code fails, or
    /// if the generated Rust program fails to run.
    ///
    /// # Panics
    ///
    /// This function will panic if the compilation of the TypeScript code fails, or if the
    /// generated Rust program fails to run.
    
    fn compute(&mut self) -> Result<Self::Output> {
        let compilation = compile_safe(&self.source)?;
        let rust = compilation.rust_string();
        let stdout = run_generated(&rust)?;

        Ok(HeavyResult { stdout, rust })
    }

    fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
        Ok(output)
    }
}

fn compile_safe(source: &str) -> Result<Compilation> {
    std::panic::catch_unwind(|| compile_typescript(source)).map_err(|err| {
        let message = if let Some(msg) = err.downcast_ref::<&str>() {
            (*msg).to_string()
        } else if let Some(msg) = err.downcast_ref::<String>() {
            msg.clone()
        } else {
            "unknown panic while compiling TypeScript".to_string()
        };

        to_napi_error(format!("ts2rust compiler panicked: {message}"))
    })
}

fn run_generated(rust: &str) -> Result<String> {
    let workspace_root = workspace_root();
    let runtime_path = workspace_root.join("old_compiler/crates/runtime");
    let target_dir = workspace_root.join("target").join("napi");
    let work_root = workspace_root.join("target").join("napi-work");

    fs::create_dir_all(&target_dir)?;
    fs::create_dir_all(&work_root)?;

    let workdir = create_workdir(&work_root)?;
    let manifest = generated_manifest(&runtime_path);

    let src_dir = workdir.join("src");
    fs::create_dir_all(&src_dir)?;
    fs::write(workdir.join("Cargo.toml"), manifest)?;
    fs::write(src_dir.join("main.rs"), rust)?;

    let output = Command::new("cargo")
        .args(["run", "--quiet", "--release"])
        .env("CARGO_TARGET_DIR", &target_dir)
        .current_dir(&workdir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(to_napi_error(format!(
            "generated program failed (artifacts kept at {}):\n{stderr}",
            workdir.display()
        )));
    }

    let _ = fs::remove_dir_all(&workdir);

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

fn generated_manifest(runtime_path: &Path) -> String {
    format!(
        "[package]\nname = \"{GENERATED_PACKAGE_NAME}\"\nversion = \"0.1.0\"\nedition = \"{GENERATED_EDITION}\"\n\n[dependencies]\nruntime = {{ path = \"{}\" }}\n\n[workspace]\n",
        runtime_path.display()
    )
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .expect("workspace root should exist")
        .to_path_buf()
}

fn create_workdir(work_root: &Path) -> std::io::Result<PathBuf> {
    let dir = work_root.join(format!("ts2rust-generated-{}", Uuid::new_v4().as_simple()));
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

fn to_napi_error(message: impl Into<String>) -> Error {
    Error::new(Status::GenericFailure, message.into())
}
