use crate::Codegen;
use ir::IrObjectProperty;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn object_literal_tokens(properties: &[IrObjectProperty]) -> TokenStream {
    let inserts: Vec<TokenStream> = properties
        .iter()
        .map(|property| {
            let key = &property.key;
            let value = property.value.codegen();
            quote! {
                map.insert(#key.to_string(), #value);
            }
        })
        .collect();

    quote! {{
        let mut map = ::std::collections::BTreeMap::<::std::string::String, runtime::value::Value>::new();
        #( #inserts )*
        runtime::value::Value::Object(map)
    }}
}

#[cfg(test)]
mod tests {
    use super::*;
    use ir::{IrExpression, IrLiteral, IrObjectProperty};
    use quote::quote;

    #[test]
    fn generates_map_construction_for_properties() {
        let tokens = object_literal_tokens(&[
            IrObjectProperty {
                key: "a".into(),
                value: IrExpression::RuntimeCall(ir::RuntimeNamespace::Value(
                    ir::ValueCall::Coerce {
                        expr: Box::new(IrExpression::Literal(IrLiteral::Number(1.0))),
                    },
                )),
            },
            IrObjectProperty {
                key: "b".into(),
                value: IrExpression::RuntimeCall(ir::RuntimeNamespace::Value(
                    ir::ValueCall::Coerce {
                        expr: Box::new(IrExpression::Identifier("value".into())),
                    },
                )),
            },
        ]);

        let expected = quote! {{
            let mut map = ::std::collections::BTreeMap::<::std::string::String, runtime::value::Value>::new();
            map.insert(
                "a".to_string(),
                runtime::value::into_value(runtime::value::Value::Number(1.0))
            );
            map.insert("b".to_string(), runtime::value::into_value(value));
            runtime::value::Value::Object(map)
        }};

        assert_eq!(tokens.to_string(), expected.to_string());
    }
}
