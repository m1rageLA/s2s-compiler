use proc_macro2::{Literal, TokenStream};
use quote::quote;
use swc_ecma_ast::{Lit, Number, Str};

pub fn emit(lit: Lit) -> TokenStream {
    match lit {
        Lit::Str(str_lit) => string(str_lit),
        Lit::Num(num_lit) => number(num_lit),
        _ => todo!("Handle other literal types as needed"),
    }
}

fn string(str_lit: Str) -> TokenStream {
    let val = str_lit.value.as_str().unwrap();
    let literal = Literal::string(val);

    quote! {
        #literal
    }
}

fn number(num_lit: Number) -> TokenStream {
    let val = num_lit.value;
    // OPT(RED): Consider handling different numeric types it might have huge performance implications
    let literal = Literal::f64_suffixed(val);

    quote! {
        #literal
    }
}
