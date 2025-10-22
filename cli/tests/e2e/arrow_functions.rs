use crate::test_utils::{get_last_lines, run_ts_program};

#[test]
fn arrow_functions_and_higher_order_calls() {
    let source = r#"
        const double = (value: number): number => value * 2;
        const increment = (value: number): number => value + 1;

        const start = 5;
        const afterDouble = double(start);
        const afterIncrement = increment(afterDouble);

        const pipeline = (value: number): number => {
            const first = increment(value);
            return double(first);
        };

        const pipelineResult = pipeline(3);

        console.log(`double=${afterDouble}`);
        console.log(`increment=${afterIncrement}`);
        console.log(`pipeline=${pipelineResult}`);
    "#;

    let stdout = run_ts_program(source);
    let lines = get_last_lines(&stdout, 3);

    assert_eq!(
        lines,
        vec!["double=10", "increment=11", "pipeline=8"],
        "unexpected stdout:\n{}",
        stdout
    );
}
