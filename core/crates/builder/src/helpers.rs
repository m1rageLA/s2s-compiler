use std::path::{Path, PathBuf};

use anyhow::{Context, Result, anyhow};
use uuid::Uuid;

use crate::{ENTRY_FUNCTION, FunctionSignature, GENERATED_EDITION};

pub fn strip_generated_main(rust: &str) -> &str {
    rust.rfind("\nfn main()")
        .or_else(|| rust.find("fn main()"))
        .map(|index| rust[..index].trim_end())
        .unwrap_or_else(|| rust.trim_end())
}

pub fn rust_type(ts_type: &str) -> Result<&'static str> {
    match ts_type.trim() {
        "number" => Ok("f64"),
        "string" => Ok("String"),
        "boolean" => Ok("bool"),
        other => Err(anyhow!("{other}")),
    }
}

pub fn generated_manifest(runtime_path: &Path, package_name: &str) -> String {
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

pub fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(3)
        .expect("builder crate should live under core/crates/builder")
        .to_path_buf()
}

pub fn runtime_path() -> PathBuf {
    workspace_root().join("core").join("crates").join("runtime")
}

pub fn artifact_id() -> String {
    format!("generated_{}", Uuid::new_v4().as_simple())
}

pub fn executable_path(target_dir: &Path, package_name: &str) -> PathBuf {
    let mut path = target_dir.join("release").join(package_name);
    if cfg!(windows) {
        path.set_extension("exe");
    }
    path
}

pub fn function_artifact_source(rust: &str, signature: &FunctionSignature) -> Result<String> {
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
