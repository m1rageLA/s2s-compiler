use once_cell::sync::Lazy;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::{Mutex, MutexGuard},
};

pub(crate) fn workspace_root() -> PathBuf {
    let output = std::process::Command::new("cargo")
        .arg("metadata")
        .arg("--format-version=1")
        .output()
        .expect("Failed to execute cargo metadata");

    let metadata: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("Failed to parse cargo metadata");

    PathBuf::from(
        metadata["workspace_root"]
            .as_str()
            .expect("No workspace root found"),
    )
}

pub(crate) fn get_last_line(stdout: &str) -> String {
    get_last_lines(stdout, 1)
        .into_iter()
        .next()
        .unwrap_or_default()
}

pub(crate) fn get_output_lines(stdout: &str) -> Vec<String> {
    stdout
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect()
}

pub(crate) fn get_last_lines(stdout: &str, count: usize) -> Vec<String> {
    if count == 0 {
        return Vec::new();
    }

    let lines = get_output_lines(stdout);
    let start = lines.len().saturating_sub(count);
    lines[start..].to_vec()
}

pub(crate) fn run_ts_program(source: &str) -> String {
    let _lock = e2e_guard();
    let fixture = TsFixture::new(source).expect("failed to write test TypeScript program");

    let output = Command::new("cargo")
        .args(["run", "--release"])
        .current_dir(fixture.root())
        .output()
        .expect("failed to run compiler");

    assert!(
        output.status.success(),
        "compilation failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    String::from_utf8_lossy(&output.stdout).into_owned()
}

static E2E_LOCK: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

pub(crate) fn e2e_guard() -> MutexGuard<'static, ()> {
    E2E_LOCK.lock().expect("e2e mutex poisoned")
}

pub(crate) struct TsFixture {
    root: PathBuf,
    program_path: PathBuf,
    original: Option<Vec<u8>>,
}

impl TsFixture {
    pub(crate) fn new(source: &str) -> std::io::Result<Self> {
        let root = workspace_root();
        let program_path = root.join("old_compiler/ts/program.ts");
        let original = fs::read(&program_path).ok();
        fs::write(&program_path, source)?;
        Ok(Self {
            root,
            program_path,
            original,
        })
    }

    pub(crate) fn root(&self) -> &Path {
        &self.root
    }
}

impl Drop for TsFixture {
    fn drop(&mut self) {
        if let Some(ref original) = self.original {
            let _ = fs::write(&self.program_path, original);
        }
    }
}
