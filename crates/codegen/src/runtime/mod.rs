pub mod array;
pub mod console;
pub mod value;

use ir::RuntimeNamespace;
use proc_macro2::TokenStream;

pub(crate) fn runtime_call_tokens(namespace: &RuntimeNamespace) -> TokenStream {
    match namespace {
        RuntimeNamespace::Console(call) => console::console_call_tokens(call),
        RuntimeNamespace::Array(call) => array::array_call_tokens(call),
        RuntimeNamespace::Value(call) => value::value_call_tokens(call),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ir::{ArrayCall, ConsoleCall, IrExpression, IrLiteral, RuntimeNamespace, ValueCall};

    #[test]
    fn dispatches_console_calls() {
        let namespace = RuntimeNamespace::Console(ConsoleCall::Log(vec![
            IrExpression::Literal(IrLiteral::Number(1.0)),
            IrExpression::Identifier("value".into()),
        ]));

        let tokens = runtime_call_tokens(&namespace);

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

    #[test]
    fn dispatches_array_calls() {
        let namespace = RuntimeNamespace::Array(ArrayCall::Push {
            target: Box::new(IrExpression::Identifier("values".into())),
            args: vec![IrExpression::RuntimeCall(RuntimeNamespace::Value(
                ValueCall::Coerce {
                    expr: Box::new(IrExpression::Literal(IrLiteral::Number(4.0))),
                },
            ))],
        });

        let tokens = runtime_call_tokens(&namespace);

        assert_eq!(
            tokens.to_string(),
            quote::quote! {
                runtime::array::push(
                    &mut values,
                    vec![runtime::value::into_value(runtime::value::Value::Number(4.0))]
                )
            }
            .to_string()
        );
    }

    #[test]
    fn dispatches_value_calls() {
        let namespace = RuntimeNamespace::Value(ValueCall::Add {
            left: Box::new(IrExpression::Literal(IrLiteral::Number(1.0))),
            right: Box::new(IrExpression::Identifier("value".into())),
        });

        let tokens = runtime_call_tokens(&namespace);

        assert_eq!(
            tokens.to_string(),
            quote::quote! {{
                let left_tmp = (runtime::value::Value::Number(1.0)).clone();
                let right_tmp = (value).clone();
                runtime::value::ops::add(left_tmp, right_tmp)
            }}
            .to_string()
        );
    }
}
