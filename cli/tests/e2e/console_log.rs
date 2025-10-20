use crate::test_utils::{TsFixture, e2e_guard, get_last_line};
use std::process::Command;

#[test]
fn console_log() {
    let _lock = e2e_guard();
    let fixture = TsFixture::new("console.log('Hello, world!');")
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
    assert_eq!(
        get_last_line(&stdout),
        "Hello, world!",
        "expected 'Hello, world!', got {}",
        stdout
    );
}
