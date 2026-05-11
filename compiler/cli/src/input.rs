use std::env;
use std::fs;
use std::path::PathBuf;

pub fn load_typescript_source() -> Result<String, String> {
    let path_arg = env::args().nth(2);
    let path: PathBuf = path_arg
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(crate::DEFAULT_TS_PATH));
    fs::read_to_string(&path).map_err(|e| format!("Failed to read {}: {e}", path.display()))
}
