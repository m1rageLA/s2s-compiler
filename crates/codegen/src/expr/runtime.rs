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

#[cfg(test)]
mod tests {
    use super::*;
    use ir::{ConsoleCall, IrExpression, IrLiteral, RuntimeNamespace};

    #[test]
    fn console_log_wraps_arguments_with_stringify() {
        let namespace = RuntimeNamespace::Console(ConsoleCall::Log(vec![
            IrExpression::Literal(IrLiteral::Number(1.0)),
            IrExpression::Identifier("value".into()),
        ]));

        let tokens = runtime_call_tokens(&namespace);
        assert_eq!(
            tokens.to_string(),
            quote::quote! {
                runtime::console::log(vec![
                    runtime::console::stringify(&(1.0)),
                    runtime::console::stringify(&(value))
                ])
            }
            .to_string()
        );
    }
}
