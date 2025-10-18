use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("cli crate is expected to have a parent directory")
        .to_path_buf()
}

#[test]
fn simple_math() {
    let root = workspace_root();
    let ts_code = "let x: number = 1 + 2; console.log(x);";
    let ts_path = root.join("ts/program.ts");
    let original_ts = fs::read(&ts_path).ok();
    fs::write(&ts_path, ts_code).expect("failed to write test TypeScript program");

    let output = Command::new("cargo")
        .args(["run", "--release"])
        .current_dir(&root)
        .output()
        .expect("failed to run compiler");

    assert!(
        output.status.success(),
        "compilation failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let last_line = stdout
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .last()
        .unwrap_or_default()
        .to_string();

    assert_eq!(last_line, "3", "expected 3, got {}", stdout);

    if let Some(original) = original_ts {
        fs::write(&ts_path, original).expect("failed to restore original ts/program.ts");
    }
}
