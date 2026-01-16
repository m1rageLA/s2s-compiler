use crate::Codegen;
use ir::{IrExpression, StringCall};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub(crate) fn string_call_tokens(call: &StringCall) -> TokenStream {
    match call {
        StringCall::Length { target } => {
            let target_tokens = value_arg_tokens(target);
            quote! {{
                runtime::string::length(#target_tokens).into_number()
            }}
        }
        StringCall::ToUpperCase { target } => unary_value_call("to_upper_case", target),
        StringCall::ToLowerCase { target } => unary_value_call("to_lower_case", target),
        StringCall::Split {
            target,
            separator,
            limit,
        } => {
            let target_tokens = value_arg_tokens(target);
            let separator_tokens = optional_value_arg_tokens(separator);
            let limit_tokens = optional_value_arg_tokens(limit);
            quote! {{
                runtime::string::split(
                    #target_tokens,
                    #separator_tokens,
                    #limit_tokens
                )
            }}
        }
        StringCall::Replace {
            target,
            pattern,
            replacement,
        } => {
            let target_tokens = value_arg_tokens(target);
            let pattern_tokens = value_arg_tokens(pattern);
            let replacement_tokens = value_arg_tokens(replacement);
            quote! {{
                runtime::string::replace(
                    #target_tokens,
                    #pattern_tokens,
                    #replacement_tokens
                )
            }}
        }
        StringCall::Includes {
            target,
            search,
            position,
        } => {
            let target_tokens = value_arg_tokens(target);
            let search_tokens = value_arg_tokens(search);
            let position_tokens = optional_value_arg_tokens(position);
            quote! {{
                runtime::string::includes(
                    #target_tokens,
                    #search_tokens,
                    #position_tokens
                )
            }}
        }
        StringCall::Concat { target, args } => {
            let target_tokens = value_arg_tokens(target);
            let args_tokens = args.iter().map(value_arg_tokens);
            quote! {{
                runtime::string::concat(
                    #target_tokens,
                    vec![#(#args_tokens),*]
                )
            }}
        }
        StringCall::Slice { target, start, end } => {
            let target_tokens = value_arg_tokens(target);
            let start_tokens = optional_value_arg_tokens(start);
            let end_tokens = optional_value_arg_tokens(end);
            quote! {{
                runtime::string::slice(
                    #target_tokens,
                    #start_tokens,
                    #end_tokens
                )
            }}
        }
        StringCall::Substr {
            target,
            start,
            length,
        } => {
            let target_tokens = value_arg_tokens(target);
            let start_tokens = optional_value_arg_tokens(start);
            let length_tokens = optional_value_arg_tokens(length);
            quote! {{
                runtime::string::substr(
                    #target_tokens,
                    #start_tokens,
                    #length_tokens
                )
            }}
        }
    }
}

fn unary_value_call(name: &str, target: &IrExpression) -> TokenStream {
    let target_tokens = value_arg_tokens(target);
    let ident = format_ident!("{}", name);
    quote! {{
        runtime::string::#ident(#target_tokens)
    }}
}

fn value_arg_tokens(expr: &IrExpression) -> TokenStream {
    let tokens = expr.codegen();
    quote! { runtime::value::into_value((#tokens).clone()) }
}

fn optional_value_arg_tokens(expr: &Option<Box<IrExpression>>) -> TokenStream {
    match expr {
        Some(expr) => {
            let tokens = expr.codegen();
            quote! { Some(runtime::value::into_value((#tokens).clone())) }
        }
        None => quote! { None },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ir::{IrExpression, IrLiteral, StringCall};

    fn string_lit(value: &str) -> IrExpression {
        IrExpression::Literal(IrLiteral::Str(value.into()))
    }

    #[test]
    fn generates_to_upper_case_call() {
        let call = StringCall::ToUpperCase {
            target: Box::new(string_lit("value")),
        };
        let tokens = string_call_tokens(&call);

        assert!(tokens.to_string().contains("runtime :: string :: to_upper_case"));
    }

    #[test]
    fn generates_split_call_with_args() {
        let call = StringCall::Split {
            target: Box::new(string_lit("items")),
            separator: Some(Box::new(string_lit(","))),
            limit: Some(Box::new(IrExpression::Literal(IrLiteral::Number(2.0)))),
        };

        let tokens = string_call_tokens(&call);

        assert!(tokens.to_string().contains("runtime :: string :: split"));
    }

    #[test]
    fn generates_length_call() {
        let call = StringCall::Length {
            target: Box::new(string_lit("abc")),
        };
        let tokens = string_call_tokens(&call);

        assert!(tokens.to_string().contains("runtime :: string :: length"));
    }

    #[test]
    fn generates_includes_call_with_position() {
        let call = StringCall::Includes {
            target: Box::new(string_lit("source")),
            search: Box::new(string_lit("c")),
            position: Some(Box::new(IrExpression::Literal(IrLiteral::Number(2.0)))),
        };

        let tokens = string_call_tokens(&call);

        assert!(tokens.to_string().contains("runtime :: string :: includes"));
    }

    #[test]
    fn generates_concat_call_with_args() {
        let call = StringCall::Concat {
            target: Box::new(string_lit("base")),
            args: vec![
                string_lit("-"),
                IrExpression::Literal(IrLiteral::Number(1.0)),
            ],
        };

        let tokens = string_call_tokens(&call);

        assert!(tokens.to_string().contains("runtime :: string :: concat"));
    }
}
