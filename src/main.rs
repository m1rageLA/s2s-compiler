fn main() {
    let x1 = "function lib(x: number, y: number): number {return x+ y}";

    let x2 = "1 + 2";
    
    let code = x2;

    let ir =lowering::ast_to_ir(&parser::ast(code));
    let ast = parser::ast(code);


    println!("{:#?}", ir);
}

//TODO имплементировать лог ошибок компилятора ts
