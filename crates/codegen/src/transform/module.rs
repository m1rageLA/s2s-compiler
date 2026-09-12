use proc_macro2::TokenStream;
use swc_ecma_ast::{Module, ModuleItem};

use super::statements;

pub fn emit(ast: Module) -> TokenStream {
    let mut output = TokenStream::new();

    for node in ast.body {
        println!("1) for - node: {:#?}", node);
        let generated = match node {
            ModuleItem::Stmt(stmt) => statements::emit(stmt),
            _ => TokenStream::new(), // Handle other module items as needed
        };
        output.extend(generated);
    }

    output
}
