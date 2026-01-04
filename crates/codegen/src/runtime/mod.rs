pub mod array;
pub mod console;
pub mod math;
pub mod string;
pub mod value;

use ir::RuntimeNamespace;
use proc_macro2::TokenStream;

pub(crate) fn runtime_call_tokens(namespace: &RuntimeNamespace) -> TokenStream {
    match namespace {
        RuntimeNamespace::Console(call) => console::console_call_tokens(call),
        RuntimeNamespace::Array(call) => array::array_call_tokens(call),
        RuntimeNamespace::Value(call) => value::value_call_tokens(call),
        RuntimeNamespace::String(call) => string::string_call_tokens(call),
        RuntimeNamespace::Math(call) => math::math_call_tokens(call),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ir::{
        ArrayCall, ConsoleCall, IrExpression, IrLiteral, MathCall, RuntimeNamespace, StringCall,
        ValueCall,
    };

    #[test]
    fn dispatches_console_calls() {
        let namespace = RuntimeNamespace::Console(ConsoleCall::Log(vec![
            IrExpression::Literal(IrLiteral::Number(1.0)),
            IrExpression::Identifier("value".into()),
        ]));

        let tokens = runtime_call_tokens(&namespace);
        assert!(tokens.to_string().contains("runtime :: console :: log"));
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
        assert!(tokens.to_string().contains("runtime :: array :: push"));
    }

    #[test]
    fn dispatches_value_calls() {
        let namespace = RuntimeNamespace::Value(ValueCall::Add {
            left: Box::new(IrExpression::Literal(IrLiteral::Number(1.0))),
            right: Box::new(IrExpression::Identifier("value".into())),
        });

        let tokens = runtime_call_tokens(&namespace);
        assert!(tokens.to_string().contains("runtime :: value :: ops :: add"));
    }

    #[test]
    fn dispatches_string_calls() {
        let namespace = RuntimeNamespace::String(StringCall::ToLowerCase {
            target: Box::new(IrExpression::Literal(IrLiteral::Str("VALUE".into()))),
        });

        let tokens = runtime_call_tokens(&namespace);
        assert!(tokens.to_string().contains("runtime :: string :: to_lower_case"));
    }

    #[test]
    fn dispatches_math_calls() {
        let namespace = RuntimeNamespace::Math(MathCall::Random);

        let tokens = runtime_call_tokens(&namespace);

        assert_eq!(
            tokens.to_string(),
            quote::quote! { runtime::math::random_number() }.to_string()
        );
    }
}
