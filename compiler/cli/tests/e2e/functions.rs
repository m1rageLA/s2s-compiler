use crate::test_utils::{get_last_lines, run_ts_program};

#[test]
fn functions_and_recursion() {
    let source = r#"
        function factorial(n: number): number {
            if (n <= 1) {
                return 1;
            }

            return n * factorial(n - 1);
        }

        function sumRange(n: number): number {
            let total: number = 0;
            for (let i = 1; i <= n; i = i + 1) {
                total = total + i;
            }
            return total;
        }

        const fact5 = factorial(5);
        const sum5 = sumRange(5);
        console.log(`fact=${fact5}`);
        console.log(`sum=${sum5}`);
    "#;

    let stdout = run_ts_program(source);
    let lines = get_last_lines(&stdout, 2);

    assert_eq!(
        lines,
        vec!["fact=120", "sum=15"],
        "unexpected stdout:\n{}",
        stdout
    );
}
