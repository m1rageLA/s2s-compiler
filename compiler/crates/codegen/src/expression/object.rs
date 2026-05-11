use crate::{Codegen, typing};
use ir::{IrExpression, IrObjectProperty, IrType, IrTypeAliasDef, RuntimeNamespace, ValueCall};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

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

pub(crate) fn object_struct_literal_tokens(
    type_id: u32,
    properties: &[IrObjectProperty],
) -> TokenStream {
    let Some(alias) = typing::lookup_type_alias(type_id) else {
        return object_literal_tokens(properties);
    };

    let struct_ident = format_ident!("{}", alias.name);
    let fields = match alias.def {
        IrTypeAliasDef::Object(fields) => fields,
        IrTypeAliasDef::Alias(_) => return object_literal_tokens(properties),
    };

    let field_tokens: Vec<TokenStream> = properties
        .iter()
        .map(|property| {
            let key = &property.key;
            let field_ident = format_ident!("{}", key);
            let field_ty = fields
                .iter()
                .find(|field| field.name == *key)
                .map(|field| field.ty)
                .unwrap_or(IrType::Any);

            let value_expr = match (&field_ty, &property.value) {
                (IrType::Any | IrType::Value, value) => value,
                (
                    _,
                    IrExpression::RuntimeCall(RuntimeNamespace::Value(ValueCall::Coerce { expr })),
                ) => expr.as_ref(),
                _ => &property.value,
            };

            let value_tokens = match field_ty {
                IrType::Object(id) => match &property.value {
                    IrExpression::Object(nested) => object_struct_literal_tokens(id, nested),
                    _ => value_expr.codegen(),
                },
                _ => value_expr.codegen(),
            };

            let expr_ty = typing::infer_expression_type(value_expr);
            let coerced = typing::coerce_to_type(value_tokens, &field_ty, expr_ty);

            quote! { #field_ident: #coerced }
        })
        .collect();

    quote! { #struct_ident { #( #field_tokens, )* } }
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
                runtime::value::into_value(1)
            );
            map.insert("b".to_string(), runtime::value::into_value(value));
            runtime::value::Value::Object(map)
        }};

        assert_eq!(tokens.to_string(), expected.to_string());
    }
}
