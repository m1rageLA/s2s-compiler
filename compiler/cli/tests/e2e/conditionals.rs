use crate::test_utils::{get_last_lines, run_ts_program};

#[test]
fn conditionals_and_templates() {
    let source = r#"
        function labelFor(score: number): string {
            if (score > 5) {
                return "high";
            }
            return "low";
        }

        const score: number = 7;
        const grade = score >= 9 ? "A" : (score >= 7 ? "B" : "C");
        const label = labelFor(score);

        const status = label === "high" ? "pass" : "fail";
        const user = "Ada";
        const message = `user=${user} grade=${grade} label=${label} status=${status}`;
        console.log(message);
    "#;

    let stdout = run_ts_program(source);
    let lines = get_last_lines(&stdout, 1);

    assert_eq!(
        lines,
        vec!["user=Ada grade=B label=high status=pass"],
        "unexpected stdout:\n{}",
        stdout
    );
}
