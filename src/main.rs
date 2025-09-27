fn main() {
    let x1 = "function lib(x: number, y: number): number {return x+ y}";

    let x2 = " let x: number; ";

    let code = x2;

    let ir = lowering::ast_to_ir(&parser::ast(code));
    let ast: swc_ecma_ast::Module = parser::ast(code);
    let rust = codegen::gen_module(&ir);

    println!("{}", rust);
}

//TODO имплементировать лог ошибок компилятора ts
