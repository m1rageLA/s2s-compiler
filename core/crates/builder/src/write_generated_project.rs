use std::{fs, path::Path};

use anyhow::{Context, Result};

use crate::helpers::{generated_manifest, runtime_path};

pub fn write_generated_project(workdir: &Path, package_name: &str, rust: &str) -> Result<()> {
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
