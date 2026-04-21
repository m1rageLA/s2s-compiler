use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::collect_stmt_tokens;
use ir::IrCatchClause;
use ir::IrStmt;

pub(crate) fn try_tokens(
    try_block: &[IrStmt],
    catch: Option<&IrCatchClause>,
    finally: Option<&[IrStmt]>,
) -> TokenStream {
    let try_tokens = collect_stmt_tokens(try_block);
    let finally_tokens = finally.map(collect_stmt_tokens).unwrap_or_default();

    let err_ident = format_ident!("ts_try_err");
    let rethrow_ident = format_ident!("ts_try_rethrow");

    let catch_body = match catch {
        Some(clause) => {
            let rendered = render_catch(clause, &err_ident);
            quote!({
                #rendered
                None
            })
        }
        None => quote!({ Some(#err_ident) }),
    };

    let finally_block = quote! { #(#finally_tokens)* };

    quote!({
        let #rethrow_ident = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            #(#try_tokens)*
        })) {
            Ok(_) => None,
            Err(#err_ident) => #catch_body,
        };
        #finally_block
        if let Some(err) = #rethrow_ident {
            std::panic::resume_unwind(err);
        }
    })
}

fn render_catch(clause: &IrCatchClause, err_ident: &proc_macro2::Ident) -> TokenStream {
    let body_tokens = collect_stmt_tokens(&clause.body);
    let binding = clause.param.as_ref().map(|name| {
        let ident = format_ident!("{}", name);
        quote! {
            let #ident = runtime::value::ops::panic_to_value(&#err_ident);
        }
    });

    quote! {
        #binding
        #(#body_tokens)*
    }
}
