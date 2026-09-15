use logger::unsupported;
use proc_macro2::TokenStream;
use quote::quote;
use swc_ecma_ast::MemberProp;

pub fn emit(member: swc_ecma_ast::MemberExpr) -> TokenStream {
    let object = super::emit(*member.obj);
    match member.prop {
        MemberProp::Ident(prop) => {
            let prop = proc_macro2::Ident::new(prop.sym.as_ref(), proc_macro2::Span::call_site());

            quote! {
                #object.#prop
            }
        }
        MemberProp::Computed(comp) => {
            let comp = super::emit(*comp.expr);
            quote! {
                #object[#comp]
            }
        }
        _ => unsupported!(member.prop), // not es5 syntax
    }
}
