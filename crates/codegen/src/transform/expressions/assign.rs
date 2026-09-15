use logger::unsupported;
use proc_macro2::TokenStream;
use quote::quote;
use swc_ecma_ast::AssignTarget;

use crate::transform::{expressions::member, identifier};

pub fn emit(assign_expr: swc_ecma_ast::AssignExpr) -> TokenStream {
    // Implement assign feature here!!!
    // handle all operators, including `=`, `+=`, `-=`, `*=`, `/=`, `%=` etc.
    // test in for (3 element)
    let left = emit_assign_left(assign_expr.left);
    let right = super::emit(*assign_expr.right);

    quote! {
        #left = #right
    }
}

fn emit_assign_left(left: AssignTarget) -> TokenStream {
    match left {
        AssignTarget::Simple(simple) => emit_simple_assign(simple),
        AssignTarget::Pat(pat) => unsupported!(pat) // not es5 syntax
    }
}

fn emit_simple_assign(simple: swc_ecma_ast::SimpleAssignTarget) -> TokenStream {
    match simple {
        swc_ecma_ast::SimpleAssignTarget::Ident(ident) => identifier::identifier(ident.id),
        swc_ecma_ast::SimpleAssignTarget::Member(member) => member::emit(member),
        _ => unsupported!(simple) // not es5 syntax
    }
}