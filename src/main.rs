fn main() {
    let x1 = "function lib(x: number, y: number): number {return x+ y}";

    let x2 = "
    let x: number = 5;
    let y: string = \"hello\";

    function add(a: number, b: number): number {
        let c: number = a + b;
        return c;
    }

    {
        let z: boolean = true;
        x = x + 1;
    }
";

    let code = x2;

    let ir = lowering::ast_to_ir(&parser::ast(code));
    let ast: swc_ecma_ast::Module = parser::ast(code);
    let rust = codegen::gen_rust();

    println!("{:#?}", rust);
}

//TODO имплементировать лог ошибок компилятора ts
