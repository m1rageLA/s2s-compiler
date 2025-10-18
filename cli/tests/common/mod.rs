use once_cell::sync::Lazy;
use std::{
    fs,
    path::{Path, PathBuf},
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
    let last_line = stdout
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .last();

    last_line.unwrap_or_default().to_string()
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
        let program_path = root.join("ts/program.ts");
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
