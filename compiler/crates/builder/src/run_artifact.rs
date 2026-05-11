use std::{
    path::Path,
    process::{Command, Stdio},
};

use anyhow::{Context, Result, bail};

pub fn run_artifact(artifact_path: &str, args_json: &str) -> Result<String> {
    let artifact = Path::new(artifact_path);
    if !artifact.exists() {
        bail!("generated artifact does not exist: {}", artifact.display());
    }

    let output = Command::new(artifact)
        .arg(args_json)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .with_context(|| format!("failed to run generated artifact {}", artifact.display()))?;

    if !output.status.success() {
        bail!(
            "generated artifact failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}
