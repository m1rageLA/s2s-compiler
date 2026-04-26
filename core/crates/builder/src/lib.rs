use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use anyhow::{Context, Result, anyhow, bail};
use serde::Deserialize;
use uuid::Uuid;

const ENTRY_FUNCTION: &str = "__ts2rust_entry";
const GENERATED_EDITION: &str = "2021";

#[derive(Debug, Deserialize)]
pub struct FunctionSignature {
    pub params: Vec<FunctionParam>,
}

impl FunctionSignature {
    pub fn from_json(json: &str) -> Result<Self> {
        serde_json::from_str(json).context("failed to parse function signature")
    }
}

#[derive(Debug, Deserialize)]
pub struct FunctionParam {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: String,
}

pub fn run_generated(rust: &str, args_json: &str) -> Result<String> {
    let workspace_root = workspace_root();
    let id = artifact_id();
    let workdir = workspace_root.join("target").join("ts2rust-run").join(&id);
    let target_dir = workspace_root.join("target").join("ts2rust-generated");

    write_generated_project(&workdir, &id, rust)?;

    let output = Command::new("cargo")
        .args(["run", "--quiet", "--release", "--"])
        .arg(args_json)
        .env("CARGO_TARGET_DIR", &target_dir)
        .current_dir(&workdir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .context("failed to run generated Cargo project")?;

    if !output.status.success() {
        bail!(
            "generated program failed (project kept at {}):\n{}",
            workdir.display(),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let _ = fs::remove_dir_all(&workdir);
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

pub fn build_function_artifact(rust: &str, signature: &FunctionSignature) -> Result<String> {
    let workspace_root = workspace_root();
    let id = artifact_id();
    let workdir = workspace_root
        .join("target")
        .join("ts2rust-artifacts")
        .join(&id);
    let target_dir = workspace_root
        .join("target")
        .join("ts2rust-artifact-target");
    let main_rs = function_artifact_source(rust, signature)?;

    write_generated_project(&workdir, &id, &main_rs)?;

    let output = Command::new("cargo")
        .args(["build", "--quiet", "--release"])
        .env("CARGO_TARGET_DIR", &target_dir)
        .current_dir(&workdir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .context("failed to build generated artifact")?;

    if !output.status.success() {
        bail!(
            "generated artifact failed to build (project kept at {}):\n{}",
            workdir.display(),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let binary = executable_path(&target_dir, &id);
    if !binary.exists() {
        bail!(
            "generated artifact was not produced at {}",
            binary.display()
        );
    }

    Ok(binary.to_string_lossy().into_owned())
}

pub fn run_artifact(artifact_path: &str, args_json: &str) -> Result<String> {
    let artifact = Path::new(artifact_path);
    if !artifact.exists() {
        bail!("generated artifact does not exist: {}", artifact.display());
    }

    let output = Command::new(artifact)
        .arg(args_json)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .with_context(|| format!("failed to run generated artifact {}", artifact.display()))?;

    if !output.status.success() {
        bail!(
            "generated artifact failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

fn function_artifact_source(rust: &str, signature: &FunctionSignature) -> Result<String> {
    let module = strip_generated_main(rust);
    let mut arg_declarations = String::new();
    let mut call_args = Vec::with_capacity(signature.params.len());

    for (index, param) in signature.params.iter().enumerate() {
        let rust_ty = rust_type(&param.ty)
            .with_context(|| format!("unsupported parameter type for {}", param.name))?;
        let arg_name = format!("arg_{index}");
        arg_declarations.push_str(&format!(
            r#"    let {arg_name}: {rust_ty} = serde_json::from_value(args.get({index}).cloned().unwrap_or_else(|| panic!("missing argument {index}"))).expect("invalid argument {index}");
"#
        ));
        call_args.push(arg_name);
    }

    let expected_len = signature.params.len();
    let call = call_args.join(", ");

    Ok(format!(
        r#"{module}

fn main() {{
    let args_json = std::env::args().nth(1).unwrap_or_else(|| "[]".to_string());
    let args: Vec<serde_json::Value> = serde_json::from_str(&args_json).expect("arguments must be a JSON array");
    if args.len() != {expected_len} {{
        panic!("expected {expected_len} arguments, got {{}}", args.len());
    }}
{arg_declarations}    let result = {ENTRY_FUNCTION}({call});
    println!("{{}}", serde_json::to_string(&result).expect("result must be JSON-serializable"));
}}
"#
    ))
}

fn strip_generated_main(rust: &str) -> &str {
    rust.rfind("\nfn main()")
        .or_else(|| rust.find("fn main()"))
        .map(|index| rust[..index].trim_end())
        .unwrap_or_else(|| rust.trim_end())
}

fn rust_type(ts_type: &str) -> Result<&'static str> {
    match ts_type.trim() {
        "number" => Ok("f64"),
        "string" => Ok("String"),
        "boolean" => Ok("bool"),
        other => Err(anyhow!("{other}")),
    }
}

fn write_generated_project(workdir: &Path, package_name: &str, rust: &str) -> Result<()> {
    let src_dir = workdir.join("src");
    fs::create_dir_all(&src_dir)
        .with_context(|| format!("failed to create {}", src_dir.display()))?;
    fs::write(
        workdir.join("Cargo.toml"),
        generated_manifest(&runtime_path(), package_name),
    )
    .with_context(|| format!("failed to write {}", workdir.join("Cargo.toml").display()))?;
    fs::write(src_dir.join("main.rs"), rust)
        .with_context(|| format!("failed to write {}", src_dir.join("main.rs").display()))?;
    Ok(())
}

fn generated_manifest(runtime_path: &Path, package_name: &str) -> String {
    format!(
        r#"[package]
name = "{package_name}"
version = "0.1.0"
edition = "{GENERATED_EDITION}"

[dependencies]
runtime = {{ path = "{}" }}
serde_json = "1"

[workspace]
"#,
        runtime_path.display()
    )
}

fn runtime_path() -> PathBuf {
    workspace_root().join("core").join("crates").join("runtime")
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(3)
        .expect("builder crate should live under core/crates/builder")
        .to_path_buf()
}

fn artifact_id() -> String {
    format!("generated_{}", Uuid::new_v4().as_simple())
}

fn executable_path(target_dir: &Path, package_name: &str) -> PathBuf {
    let mut path = target_dir.join("release").join(package_name);
    if cfg!(windows) {
        path.set_extension("exe");
    }
    path
}
