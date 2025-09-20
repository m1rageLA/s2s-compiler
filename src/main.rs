fn main() {
    // lowering::ast_to_ir(&parser::ast("let x: number = 10"));
    let ast = parser::ast("function lib(x: number, y: number) {return x+ y}");
    println!("{:#?}", ast);
}

//TODO имплементировать лог ошибок компилятора ts