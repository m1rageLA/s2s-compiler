use crate::test_utils::{get_last_lines, run_ts_program};

#[test]
fn switch_for_in_and_sequence() {
    let source = r#"
        const input = 2;
        let result = "";
        switch (input) {
            case 1:
                result += "one";
                break;
            case 2:
                result += "two";
            case 3:
                result += "three";
                break;
            default:
                result += "default";
        }

        const obj = { a: 1, b: 2 };
        let keys = "";
        for (const key in obj) {
            keys += key;
        }

        const arr = [10, 20];
        let arrKeys = "";
        for (let idx in arr) {
            arrKeys += idx;
        }

        const seq = (result, keys, arrKeys);

        console.log(result);
        console.log(keys);
        console.log(arrKeys);
        console.log(seq);
    "#;

    let stdout = run_ts_program(source);
    let lines = get_last_lines(&stdout, 4);

    assert_eq!(
        lines,
        vec!["twothree", "ab", "01", "01"],
        "unexpected stdout:\n{}",
        stdout
    );
}

#[test]
fn try_catch_finally_and_throw() {
    let source = r#"
        function demo() {
            try {
                throw "boom";
            } catch (err) {
                console.log(`caught=${err}`);
            } finally {
                console.log("finally");
            }
        }

        demo();
    "#;

    let stdout = run_ts_program(source);
    let lines = get_last_lines(&stdout, 2);

    assert_eq!(
        lines,
        vec!["caught=boom", "finally"],
        "unexpected stdout:\n{}",
        stdout
    );
}

#[test]
fn prefix_updates_and_unary_ops() {
    let source = r#"
        let value = 1;
        const pre = ++value;
        const post = value++;
        const bit = ~pre;
        const typed = typeof value;
        const nothing = void value;
        const obj: any = { temp: 1 };
        const removed = delete obj.temp;
        const seq = (value, pre, post);

        console.log(`value=${value}`);
        console.log(`pre=${pre},post=${post}`);
        console.log(`bit=${bit}`);
        console.log(`typeof=${typed}`);
        console.log(`void=${nothing}`);
        console.log(`deleted=${removed}`);
        console.log(`seq=${seq}`);
        console.log(`null=${null}`);
    "#;

    let stdout = run_ts_program(source);
    let lines = get_last_lines(&stdout, 8);

    assert_eq!(
        lines,
        vec![
            "value=3",
            "pre=2,post=2",
            "bit=-3",
            "typeof=number",
            "void=undefined",
            "deleted=true",
            "seq=2",
            "null=null"
        ],
        "unexpected stdout:\n{}",
        stdout
    );
}
