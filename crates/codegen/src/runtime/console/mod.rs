mod log;

use ir::ConsoleCall;
use proc_macro2::TokenStream;

pub(crate) fn console_call_tokens(call: &ConsoleCall) -> TokenStream {
    match call {
        ConsoleCall::Log(args) => log::log_tokens(args),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ir::{IrExpression, IrLiteral};

    #[test]
    fn console_log_wraps_arguments_with_stringify() {
        let call = ConsoleCall::Log(vec![
            IrExpression::Literal(IrLiteral::Number(1.0)),
            IrExpression::Identifier("value".into()),
        ]);

        let tokens = console_call_tokens(&call);
        assert_eq!(
            tokens.to_string(),
            quote::quote! {
                runtime::console::log(vec![
                    runtime::console::stringify_any(&(runtime::value::Value::Number(1.0))),
                    runtime::console::stringify_any(&(value))
                ])
            }
            .to_string()
        );
    }
}
