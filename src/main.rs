

fn main() {
    lowering::ast_to_ir(&parser::ast("let x = 10"));
}
