use ir::{ConsoleCall, RuntimeNamespace};
use proc_macro2::TokenStream;
use quote::quote;

use crate::Codegen;

pub(crate) fn runtime_call_tokens(namespace: &RuntimeNamespace) -> TokenStream {
    match namespace {
        RuntimeNamespace::Console(call) => console_call_tokens(call),
    }
}

fn console_call_tokens(call: &ConsoleCall) -> TokenStream {
    match call {
        ConsoleCall::Log(args) => {
            let arg_tokens: Vec<TokenStream> = args
                .iter()
                .map(|arg| {
                    let expr = arg.codegen();
                    quote! { runtime::console::stringify(&(#expr)) }
                })
                .collect();
            quote! { runtime::console::log(vec![ #( #arg_tokens ),* ]) }
        }
    }
}
