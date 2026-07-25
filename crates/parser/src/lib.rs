use swc_common::sync::Lrc;
use swc_common::{FileName, SourceMap};

fn parse() -> () {

    // SourceMap manages source files and resolves byte positions to source locations
    // It can inform us about exact position of Error, element, code etc.
    // In the next generation we can add linter using this SourceMap, because we will be able to locate exact 'heavy' function or part of code
    // OR we will be able to show users what is not suppoertd for now
    let source_map: Lrc<SourceMap> = Default::default();
    let program = source_map.new_source_file(FileName::Custom("_parse_file.ts".into()).into(), "function foo() {}");
}

#[cfg(test)]
mod tests {
    use std::println;

    use super::*;

    #[test]
    fn test_parse() {
        let res = parse();
        println!("ANSWER: {:?}", res)
    }
}
