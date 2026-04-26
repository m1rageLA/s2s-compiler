use anyhow::Result;

pub fn compile_to_rust(source: &str) -> Result<String> {
    let compilation = ts2rust_core::compile_typescript(source);
    Ok(compilation.rust_string())
}

pub fn compile_and_execute(source: &str, args_json: &str) -> Result<String> {
    let rust = compile_to_rust(source)?;
    builder::run_generated(&rust, args_json)
}

pub fn compile_function_artifact(source: &str, signature_json: &str) -> Result<String> {
    let rust = compile_to_rust(source)?;
    let signature = builder::FunctionSignature::from_json(signature_json)?;
    builder::build_function_artifact(&rust, &signature)
}

pub fn call_artifact(artifact_path: &str, args_json: &str) -> Result<String> {
    builder::run_artifact(artifact_path, args_json)
}
