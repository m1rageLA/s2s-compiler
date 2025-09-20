fn main() {
    let ir =lowering::ast_to_ir(&parser::ast("function lib(x: number, y: number) {}"));
    // let ast = parser::ast("function lib(x: number, y: number) {return x+ y}");
    println!("{:#?}", ir);
}

//TODO имплементировать лог ошибок компилятора ts