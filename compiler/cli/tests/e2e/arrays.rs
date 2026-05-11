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

#[test]
fn array_map_supports_dynamic_callbacks() {
    let source = r#"
        const numbers = [1, 2, 3];
        const incremented = numbers.map((value) => value + 1);
        console.log(incremented[0]);
        console.log(incremented[1]);
        console.log(incremented[2]);

        const stringified = numbers.map((value) => `#${value}`);
        console.log(stringified[0]);
        console.log(stringified[1]);
        console.log(stringified[2]);
    "#;

    let stdout = run_ts_program(source);
    let lines = get_last_lines(&stdout, 6);

    assert_eq!(
        lines,
        vec!["2", "3", "4", "#1", "#2", "#3"],
        "unexpected stdout:\n{}",
        stdout
    );
}

#[test]
fn array_filter_handles_boolean_predicates() {
    let source = r#"
        const values = [1, 2, 3, 4];
        const evens = values.filter((value) => value % 2 === 0);
        console.log(evens.length);
        console.log(evens[0]);
        console.log(evens[1]);

        const greaterThanTwo = values.filter((value) => value > 2);
        console.log(greaterThanTwo.length);
        console.log(greaterThanTwo[0]);
        console.log(greaterThanTwo[1]);
    "#;

    let stdout = run_ts_program(source);
    let lines = get_last_lines(&stdout, 6);

    assert_eq!(
        lines,
        vec!["2", "2", "4", "2", "3", "4"],
        "unexpected stdout:\n{}",
        stdout
    );
}
