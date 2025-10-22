use crate::test_utils::{get_last_lines, run_ts_program};

#[test]
fn loops_cover_all_constructs() {
    let source = r#"
        let countdown: number = 3;
        while (countdown > 0) {
            console.log(`while:${countdown}`);
            countdown = countdown - 1;
        }

        let seen: number = 0;
        do {
            console.log(`do:${seen}`);
            seen += 1;
        } while (seen < 2);

        let total: number = 0;
        for (let i = 0; i < 3; i = i + 1) {
            total += i;
        }

        console.log(`for:${total}`);
    "#;

    let stdout = run_ts_program(source);
    let lines = get_last_lines(&stdout, 6);

    assert_eq!(
        lines,
        vec!["while:3", "while:2", "while:1", "do:0", "do:1", "for:3"],
        "unexpected stdout:\n{}",
        stdout
    );
}
