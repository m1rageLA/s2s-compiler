fn main() {
    let code = "function lib(x: number, y: number): number {return x+ y}";
    

    let ir =lowering::ast_to_ir(&parser::ast(code));
    let ast = parser::ast(code);


    println!("{:#?}", ir);
}

//TODO имплементировать лог ошибок компилятора ts
