use quote::quote;

use crate::transform::{expressions, statements};

/// Emits a JavaScript `do … while` as a Rust `loop`.
///
/// ES5 normalization retains `DoWhileStmt`, so this cannot be delegated to the
/// ordinary `while` emitter.  Keeping the test after the body also preserves
/// the essential do-while guarantee that the body runs at least once.
pub fn emit(do_while: swc_ecma_ast::DoWhileStmt) -> proc_macro2::TokenStream {
    let test = expressions::emit(*do_while.test);
    let body = statements::emit(*do_while.body);

    quote! {
        loop {
            { #body }
            if !(#test) {
                break;
            }
        }
    }
}
