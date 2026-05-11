use crate::{
    helpers::{artifact_id, workspace_root},
    write_generated_project::write_generated_project,
};
use anyhow::{Context, Result, bail};

use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

pub fn build_llvm(rust: &str) -> Result<String> {
    let workspace_root = workspace_root();
    let id = artifact_id();
    let workdir = workspace_root.join("target").join("ts2rust-llvm").join(&id);
    let target_dir = workspace_root.join("target").join("ts2rust-llvm-target");

    write_generated_project(&workdir, &id, rust)?;

    let output = Command::new("cargo")
        .args(["rustc", "--quiet", "--release", "--", "--emit=llvm-ir"])
        .env("CARGO_TARGET_DIR", &target_dir)
        .current_dir(&workdir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .context("failed to compile generated Cargo project to LLVM IR")?;

    if !output.status.success() {
        bail!(
            "generated LLVM build failed (project kept at {}):\n{}",
            workdir.display(),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let llvm_ir_path = find_llvm_ir_file(&target_dir, &id)?;
    let llvm_ir = fs::read_to_string(&llvm_ir_path)
        .with_context(|| format!("failed to read {}", llvm_ir_path.display()))?;

    let _ = fs::remove_dir_all(&workdir);
    Ok(llvm_ir)
}

fn find_llvm_ir_file(target_dir: &Path, package_name: &str) -> Result<PathBuf> {
    let mut stack = vec![target_dir.to_path_buf()];

    while let Some(dir) = stack.pop() {
        for entry in
            fs::read_dir(&dir).with_context(|| format!("failed to read {}", dir.display()))?
        {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                stack.push(path);
                continue;
            }

            if path.extension().and_then(|ext| ext.to_str()) != Some("ll") {
                continue;
            }

            let Some(file_name) = path.file_name().and_then(|name| name.to_str()) else {
                continue;
            };

            if file_name.starts_with(package_name) {
                return Ok(path);
            }
        }
    }

    bail!(
        "generated LLVM IR was not produced under {}",
        target_dir.display()
    )
}
