use ir::{IrExpression, IrPostfixOp};
use proc_macro2::{Literal, Span, TokenStream};
use quote::{format_ident, quote};

use crate::Codegen;

pub(crate) fn postfixunary_tokens(left: Box<IrExpression>, op: IrPostfixOp) -> TokenStream {
    if let IrExpression::Member { object, property } = left.as_ref() {
        return member_postfix_tokens(object.as_ref(), property, op);
    }

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

fn member_postfix_tokens(object: &IrExpression, property: &str, op: IrPostfixOp) -> TokenStream {
    let object_tokens = object.codegen();
    let property_literal = Literal::string(property);
    let property_literal_for_set = property_literal.clone();

    let op_fn = match op {
        IrPostfixOp::Increment => quote!(runtime::value::ops::add),
        IrPostfixOp::Decrement => quote!(runtime::value::ops::sub),
    };

    quote!({
        let ts_2_rs_target = &mut #object_tokens;
        let ts_2_rs_current = runtime::value::ops::get_property((*ts_2_rs_target).clone(), #property_literal);
        let ts_2_rs_new = #op_fn(ts_2_rs_current.clone(), runtime::value::Value::Number(1.0));
        runtime::value::ops::set_property_in_place(ts_2_rs_target, #property_literal_for_set, ts_2_rs_new);
        ts_2_rs_current
    })
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
