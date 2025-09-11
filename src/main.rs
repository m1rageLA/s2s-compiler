use ir::*; // Declare `parser` as a local module

fn main() {
    lowering::ast_to_ir(&parser::ast());
}
