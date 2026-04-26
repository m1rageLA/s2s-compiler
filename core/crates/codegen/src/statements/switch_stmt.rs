use ir::{IrExpression, IrSwitchCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::collect_stmt_tokens;
use super::label::label_lifetime;
use crate::Codegen;

pub(crate) fn switch_tokens(
    discriminant: &IrExpression,
    cases: &[IrSwitchCase],
    label: Option<&str>,
) -> TokenStream {
    let disc_ident = format_ident!("ts_switch_value");
    let matched_ident = format_ident!("ts_switch_matched");
    let disc_tokens = discriminant.codegen();
    let lifetime = label.map(label_lifetime);

    let rendered_cases: Vec<TokenStream> = cases
        .iter()
        .map(|case| render_case(case, &disc_ident, &matched_ident))
        .collect();

    match lifetime {
        Some(lt) => quote!({
            let #disc_ident = runtime::value::into_value((#disc_tokens).clone());
            let mut #matched_ident = false;
            #lt: loop {
                #(#rendered_cases)*
                break #lt ::std::panic::panic_any("unmatched switch");
            }
        }),
        None => quote!({
            let #disc_ident = runtime::value::into_value((#disc_tokens).clone());
            let mut #matched_ident = false;
            loop {
                #(#rendered_cases)*
                break ::std::panic::panic_any("unmatched switch");
            }
        }),
    }
}

fn render_case(
    case: &IrSwitchCase,
    disc_ident: &proc_macro2::Ident,
    matched_ident: &proc_macro2::Ident,
) -> TokenStream {
    let body = collect_stmt_tokens(&case.consequent);
    match &case.test {
        Some(test) => {
            let test_tokens = test.codegen();
            quote! {
                if !#matched_ident {
                    #matched_ident = runtime::value::ops::strict_equal(
                        #disc_ident.clone(),
                        runtime::value::into_value((#test_tokens).clone())
                    );
                }
                if #matched_ident {
                    #(#body)*
                }
            }
        }
        None => quote! {
            if #matched_ident || { #matched_ident = true; true } {
                #(#body)*
            }
        },
    }
}
