use std::{
    fs,
    process::{Command, Stdio},
};

use anyhow::{Context, Result, bail};

use crate::{
    helpers::{artifact_id, workspace_root},
    write_generated_project::write_generated_project,
};

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
