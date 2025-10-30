use ir::{IrExpression, IrPostfixOp};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};

use crate::Codegen;

pub(crate) fn postfixunary_tokens(left: Box<IrExpression>, op: IrPostfixOp) -> TokenStream {
    let left = left.codegen();
    let temp = format_ident!("ts_2_rs", span = Span::mixed_site());

    match op {
        IrPostfixOp::Increment => {
            quote! ({
                let ts_2_rs_target = &mut #left;
                let #temp = (*ts_2_rs_target).clone();
                let ts_2_rs_new = runtime::value::ops::add(
                    #temp.clone(),
                    runtime::value::Value::Number(1.0)
                );
                *ts_2_rs_target = ts_2_rs_new;
                #temp
            })
        }
        IrPostfixOp::Decrement => {
            quote! ({
                let ts_2_rs_target = &mut #left;
                let #temp = (*ts_2_rs_target).clone();
                let ts_2_rs_new = runtime::value::ops::sub(
                    #temp.clone(),
                    runtime::value::Value::Number(1.0)
                );
                *ts_2_rs_target = ts_2_rs_new;
                #temp
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    #[test]
    fn postfix_increment_preserves_original_value() {
        let tokens = postfixunary_tokens(
            Box::new(IrExpression::Identifier("counter".into())),
            IrPostfixOp::Increment,
        );

        assert_eq!(
            tokens.to_string(),
            quote!({
                let ts_2_rs_target = &mut counter;
                let ts_2_rs = (*ts_2_rs_target).clone();
                let ts_2_rs_new =
                    runtime::value::ops::add(ts_2_rs.clone(), runtime::value::Value::Number(1.0));
                *ts_2_rs_target = ts_2_rs_new;
                ts_2_rs
            })
            .to_string()
        );
    }

    #[test]
    fn postfix_decrement_updates_value_after_snapshot() {
        let tokens = postfixunary_tokens(
            Box::new(IrExpression::Identifier("index".into())),
            IrPostfixOp::Decrement,
        );

        assert_eq!(
            tokens.to_string(),
            quote!({
                let ts_2_rs_target = &mut index;
                let ts_2_rs = (*ts_2_rs_target).clone();
                let ts_2_rs_new =
                    runtime::value::ops::sub(ts_2_rs.clone(), runtime::value::Value::Number(1.0));
                *ts_2_rs_target = ts_2_rs_new;
                ts_2_rs
            })
            .to_string()
        );
    }
}
