use crate::test_utils::{get_last_line, get_output_lines, run_ts_program};

#[test]
fn dynamic_binary_operations_match_js_semantics() {
    let output = run_ts_program(
        r#"
        console.log("string" + "string");
        console.log("5" - 2);
        console.log("5" + 2);
        console.log(10 / "2");
        console.log("5" == 5);
        console.log("5" === 5);
        console.log(3 < "10");
    "#,
    );

    let lines = get_output_lines(&output);
    let tail = &lines[lines.len().saturating_sub(7)..];

    assert_eq!(tail[0], "stringstring", "expected string concatenation");
    assert_eq!(tail[1], "3", "expected numeric subtraction");
    assert_eq!(tail[2], "52", "expected string-number addition");
    assert_eq!(tail[3], "5", "expected string division");
    assert_eq!(tail[4], "true", "expected loose equality result");
    assert_eq!(tail[5], "false", "expected strict equality result");
    assert_eq!(tail[6], "true", "expected relational comparison result");
}

#[test]
fn value_coercion_keeps_runtime_addition() {
    let output = run_ts_program(
        r#"
        function makeValue(): any {
            const inner = "foo" + "bar";
            return inner;
        }

        const value = makeValue();
        console.log(value);
    "#,
    );

    assert_eq!(
        get_last_line(&output),
        "foobar",
        "expected concatenated string, got {output}"
    );
}
