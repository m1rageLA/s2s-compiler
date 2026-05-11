use napi_derive::napi;

#[napi]
pub fn compile_to_rust(source: String) -> napi::Result<String> {
    api::compile_to_rust(&source).map_err(|error| napi::Error::from_reason(error.to_string()))
}

#[napi]
pub fn compile_and_execute(source: String, args_json: String) -> napi::Result<String> {
    api::compile_and_execute(&source, &args_json)
        .map_err(|error| napi::Error::from_reason(error.to_string()))
}

#[napi]
pub fn compile_function(source: String, signature_json: String) -> napi::Result<String> {
    api::compile_function_artifact(&source, &signature_json)
        .map_err(|error| napi::Error::from_reason(error.to_string()))
}

#[napi]
pub fn call_artifact(artifact_path: String, args_json: String) -> napi::Result<String> {
    api::call_artifact(&artifact_path, &args_json)
        .map_err(|error| napi::Error::from_reason(error.to_string()))
}

#[napi(js_name = "compileToLLVM")]
pub fn compile_to_llvm(source: String) -> napi::Result<String> {
    api::compile_to_llvm(&source).map_err(|error| napi::Error::from_reason(error.to_string()))
}
