use std::process::{Command, Stdio};

use anyhow::{Context, Result, bail};

use crate::{
    FunctionSignature,
    helpers::{artifact_id, executable_path, function_artifact_source, workspace_root},
    write_generated_project::write_generated_project,
};

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
