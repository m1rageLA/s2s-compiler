use crate::value::Value;

pub fn log(args: Vec<String>) {
    let parts: Vec<String> = args.into_iter().map(|v| v.to_string()).collect();
    println!("{}", parts.join(" "));
}
