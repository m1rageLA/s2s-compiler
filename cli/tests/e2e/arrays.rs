use crate::test_utils::{get_last_lines, run_ts_program};

#[test]
fn array_runtime_operations() {
    let source = r#"
        let values: number[] = [1, 2];
        const lenAfterPush = values.push(3, 4);
        const thirdValue = values[2];
        const finalLength = values.length;

        console.log(`len=${finalLength}`);
        console.log(`afterPush=${lenAfterPush}`);
        console.log(`third=${thirdValue}`);
    "#;

    let stdout = run_ts_program(source);
    let lines = get_last_lines(&stdout, 3);

    assert_eq!(
        lines,
        vec!["len=4", "afterPush=4", "third=3"],
        "unexpected stdout:\n{}",
        stdout
    );
}
