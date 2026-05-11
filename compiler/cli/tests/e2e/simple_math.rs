use crate::test_utils::{TsFixture, e2e_guard, get_last_line};
use std::process::Command;

#[test]
fn simple_math() {
    let _lock = e2e_guard();
    let fixture = TsFixture::new("let x: number = 1 + 2; console.log(x);")
        .expect("failed to write test TypeScript program");

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

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert_eq!(get_last_line(&stdout), "3", "expected 3, got {}", stdout);
}
